use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use cpal::traits::HostTrait;

use log::{debug, log_enabled, Level};

use rodio::queue;
use rodio::source::{Amplify, Pausable, Source, Stoppable};
use rodio::{Decoder, OutputStream};

use dizi_lib::error::DiziResult;
use dizi_lib::song::Song;

use crate::config;
use crate::events::{ServerEvent, ServerEventSender};

#[derive(Clone, Debug)]
pub enum PlayerRequest {
    Play(Song),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
    //    AddListener(ServerEventSender),
    //    ClearListeners,
}

pub struct PlayerStream {
    pub volume: f32,
    pub event_tx: ServerEventSender,
    pub player_res_tx: mpsc::Sender<DiziResult<()>>,
    pub player_req_rx: mpsc::Receiver<PlayerRequest>,
    pub source_tx: Option<mpsc::Sender<PlayerRequest>>,
    pub receiver: Option<mpsc::Receiver<()>>,
}

impl PlayerStream {
    pub fn new(
        volume: f32,
        event_tx: ServerEventSender,
        player_res_tx: mpsc::Sender<DiziResult<()>>,
        player_req_rx: mpsc::Receiver<PlayerRequest>,
    ) -> Self {
        Self {
            volume,
            event_tx,
            player_res_tx,
            player_req_rx,
            source_tx: None,
            receiver: None,
        }
    }

    pub fn player_req(&self) -> &mpsc::Receiver<PlayerRequest> {
        &self.player_req_rx
    }

    pub fn player_res(&self) -> &mpsc::Sender<DiziResult<()>> {
        &self.player_res_tx
    }

    pub fn pause(&self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        if let Some(source_tx) = self.source_tx.as_ref() {
            let _ = source_tx.send(PlayerRequest::Pause)?;
        }
        Ok(())
    }
    pub fn resume(&self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        if let Some(source_tx) = self.source_tx.as_ref() {
            source_tx.send(PlayerRequest::Resume)?;
        }
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        if let Some(source_tx) = self.source_tx.as_ref() {
            source_tx.send(PlayerRequest::Stop)?;
        }
        Ok(())
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        if let Some(source_tx) = self.source_tx.as_ref() {
            let _ = source_tx.send(PlayerRequest::SetVolume(volume));
        }
    }

    pub fn play(
        &mut self,
        queue_tx: &queue::SourcesQueueInput<f32>,
        path: &Path,
    ) -> DiziResult<mpsc::Receiver<()>> {
        const POLL_RATE: Duration = Duration::from_millis(200);
        const UPDATE_RATE: Duration = Duration::from_secs(1);

        self.stop();

        let mut duration_played = Duration::from_secs(0);
        let mut update_tracker = Duration::from_secs(0);

        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        let event_tx = self.event_tx.clone();

        // channel for controlling source
        let (source_tx, source_rx): (mpsc::Sender<PlayerRequest>, mpsc::Receiver<PlayerRequest>) =
            mpsc::channel();

        let mut paused = false;
        let source = Decoder::new(buffer)?
            .amplify(self.volume)
            .pausable(false)
            .stoppable()
            .periodic_access(POLL_RATE, move |source| {
                if !paused {
                    update_tracker += POLL_RATE;
                    if update_tracker >= UPDATE_RATE {
                        duration_played += update_tracker;
                        update_tracker = Duration::from_secs(0);
                        let _ = event_tx.send(ServerEvent::PlayerProgressUpdate(duration_played));
                    }
                }
                if let Ok(msg) = source_rx.try_recv() {
                    process_msg(msg, source, &source_rx, &mut paused);
                }
            })
            .convert_samples();

        self.source_tx = Some(source_tx);
        let finish_signal = queue_tx.append_with_signal(source);
        Ok(finish_signal)
    }
}

pub fn process_msg(
    msg: PlayerRequest,
    source: &mut Stoppable<Pausable<Amplify<Decoder<BufReader<File>>>>>,
    source_rx: &mpsc::Receiver<PlayerRequest>,
    paused: &mut bool,
) {
    match msg {
        PlayerRequest::Pause => {
            source.inner_mut().set_paused(true);
            *paused = true;
            while let Ok(msg) = source_rx.recv() {
                process_msg(msg.clone(), source, source_rx, paused);
                match msg {
                    PlayerRequest::Resume => break,
                    _ => {}
                }
            }
        }
        PlayerRequest::Resume => {
            source.inner_mut().set_paused(false);
            *paused = false;
        }
        PlayerRequest::SetVolume(volume) => {
            source.inner_mut().inner_mut().set_factor(volume);
        }
        PlayerRequest::Stop => {
            source.stop();
            *paused = true;
        }
        _ => {}
    }
}

pub fn player_stream(
    volume: f32,
    config_t: config::AppConfig,
    player_res_tx: mpsc::Sender<DiziResult<()>>,
    player_req_rx: mpsc::Receiver<PlayerRequest>,
    event_tx: ServerEventSender,
) -> DiziResult<()> {
    let mut player_stream = PlayerStream::new(volume, event_tx, player_res_tx, player_req_rx);

    let audio_device = get_default_output_device(config_t.server_ref().audio_system);
    let (_stream, stream_handle) = OutputStream::try_from_device(&audio_device)?;

    let (queue_tx, queue_rx) = rodio::queue::queue(true);
    let _ = stream_handle.play_raw(queue_rx);

    let stream_listeners: Arc<Mutex<Vec<ServerEventSender>>> = Arc::new(Mutex::new(vec![]));
    let mut done_listener: Option<thread::JoinHandle<()>> = None;

    while let Ok(msg) = player_stream.player_req().recv() {
        match msg {
            PlayerRequest::Play(song) => {
                // Before playing new song, make sure to clear any listeners waiting for previous
                // song to finish. This prevents a loop where new song triggers the end of previous
                // song which triggers a new song, and repeat.
                match stream_listeners.lock() {
                    Ok(mut vec) => vec.clear(),
                    _ => {}
                }

                let res = player_stream.play(&queue_tx, song.file_path());

                match res {
                    Ok(receiver) => {
                        // wait for previous listener (if any) to finish sending messages to listeners
                        // before repopulating listeners list for new song
                        let prev_listener = done_listener.take();
                        if let Some(prev_listener) = prev_listener {
                            prev_listener.join();
                        }

                        // spawn new listening thread for new song
                        let stream_listeners2 = stream_listeners.clone();
                        let listener = thread::spawn(move || {
                            receiver.recv();
                            if let Ok(stream_listeners) = stream_listeners2.lock() {
                                for stream_listener in stream_listeners.iter() {
                                    (*stream_listener).send(ServerEvent::PlayerDone);
                                }
                            }
                        });
                        done_listener = Some(listener);

                        // add server events to listeners
                        match stream_listeners.lock() {
                            Ok(mut vec) => vec.push(player_stream.event_tx.clone()),
                            _ => {}
                        }
                        player_stream.player_res().send(Ok(()))?;
                    }
                    Err(e) => player_stream.player_res().send(Err(e))?,
                };
            }
            PlayerRequest::Pause => {
                player_stream.pause();
                player_stream.player_res().send(Ok(()))?;
            }
            PlayerRequest::Stop => {
                player_stream.stop();
                player_stream.player_res().send(Ok(()))?;
            }
            PlayerRequest::Resume => {
                player_stream.resume();
                player_stream.player_res().send(Ok(()))?;
            }
            PlayerRequest::SetVolume(volume) => {
                player_stream.set_volume(volume);
                player_stream.player_res().send(Ok(()))?;
            }
            s => {
                if log_enabled!(Level::Debug) {
                    debug!("Not implemented '{:?}'", s);
                }
            }
        }
    }
    Ok(())
}

pub fn get_default_output_device(host_id: cpal::HostId) -> cpal::Device {
    if log_enabled!(Level::Debug) {
        debug!("Available audio systems:");
        for host in cpal::available_hosts() {
            debug!("host: {:?}", host);
        }
    }
    let host = cpal::host_from_id(
        cpal::available_hosts()
            .into_iter()
            .find(|id| *id == host_id)
            .unwrap(),
    )
    .unwrap_or_else(|_| cpal::default_host());

    let device = host.default_output_device();
    device.unwrap()
}
