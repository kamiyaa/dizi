use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait};

use rodio::queue;
use rodio::source::{Amplify, Pausable, PeriodicAccess, Source, Stoppable};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use dizi_lib::error::DiziResult;
use dizi_lib::song::Song;

use crate::config;
use crate::events::{ClientEvent, ClientEventSender};

#[derive(Clone, Debug)]
pub enum PlayerRequest {
    Play(Song),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
}

pub type RodioSource = Decoder<BufReader<File>>;
pub type RodioControllableSource = Pausable<Amplify<Stoppable<RodioSource>>>;
pub type RodioDecoder =
    PeriodicAccess<RodioControllableSource, FnMut(&mut RodioControllableSource)>;

pub type SourceThread = thread::JoinHandle<DiziResult<mpsc::Receiver<()>>>;

pub struct PlayerStream {
    pub player_res_tx: mpsc::Sender<DiziResult<()>>,
    pub player_req_rx: mpsc::Receiver<PlayerRequest>,
    pub event_tx: ClientEventSender,
    pub source_tx: Option<mpsc::Sender<PlayerRequest>>,
    pub receiver: Option<mpsc::Receiver<()>>,
}

impl PlayerStream {
    pub fn new(
        player_res_tx: mpsc::Sender<DiziResult<()>>,
        player_req_rx: mpsc::Receiver<PlayerRequest>,
        event_tx: ClientEventSender,
    ) -> Self {
        Self {
            player_res_tx,
            player_req_rx,
            event_tx,
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

    pub fn pause(&self) {
        if let Some(source_tx) = self.source_tx.as_ref() {
            source_tx.send(PlayerRequest::Pause);
        }
    }
    pub fn resume(&self) {
        if let Some(source_tx) = self.source_tx.as_ref() {
            source_tx.send(PlayerRequest::Resume);
        }
    }
    pub fn stop(&mut self) {
        let source_tx = self.source_tx.take();
        if let Some(source_tx) = source_tx {
            source_tx.send(PlayerRequest::Stop);
        }
        let source_thread = self.receiver.take();
    }

    pub fn set_volume(&self, volume: f32) {
        if let Some(source_tx) = self.source_tx.as_ref() {
            source_tx.send(PlayerRequest::SetVolume(volume));
        }
    }

    pub fn play(
        &mut self,
        queue_tx: &queue::SourcesQueueInput<f32>,
        path: &Path,
    ) -> DiziResult<()> {
        self.stop();

        const POLL_RATE: Duration = Duration::from_millis(200);
        const UPDATE_RATE: Duration = Duration::from_secs(1);

        let mut duration_played = Duration::from_secs(0);
        let mut update_tracker = Duration::from_secs(0);

        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        let event_tx2 = self.event_tx.clone();
        let (source_tx, source_rx): (mpsc::Sender<PlayerRequest>, mpsc::Receiver<PlayerRequest>) =
            mpsc::channel();

        let source = Decoder::new(buffer)?
            .stoppable()
            .amplify(1.0)
            .pausable(false)
            .periodic_access(POLL_RATE, move |source| {
                update_tracker += POLL_RATE;
                if update_tracker >= UPDATE_RATE {
                    duration_played += update_tracker;
                    update_tracker = Duration::from_secs(0);
                    eprintln!("Played {:?}", duration_played);
                    event_tx2.send(ClientEvent::PlayerProgressUpdate(duration_played));
                }

                if let Ok(msg) = source_rx.try_recv() {
                    match msg {
                        PlayerRequest::Pause => {
                            source.set_paused(true);
                        }
                        PlayerRequest::Resume => {
                            source.set_paused(false);
                        }
                        PlayerRequest::SetVolume(volume) => {
                            source.inner_mut().set_factor(volume);
                        }
                        PlayerRequest::Stop => {
                            source.inner_mut().inner_mut().stop();
                        }
                        _ => {}
                    }
                }
            })
            .convert_samples();
        self.source_tx = Some(source_tx);
        queue_tx.append_with_signal(source);
        Ok(())
    }
}

pub fn player_stream(
    config_t: config::AppConfig,
    player_res_tx: mpsc::Sender<DiziResult<()>>,
    player_req_rx: mpsc::Receiver<PlayerRequest>,
    event_tx: ClientEventSender,
) -> DiziResult<()> {
    let mut player_stream = PlayerStream::new(player_res_tx, player_req_rx, event_tx);

    let audio_device = get_default_output_device(config_t.server_ref().audio_system);
    let (stream, stream_handle) = OutputStream::try_from_device(&audio_device)?;

    let (queue_tx, queue_rx) = rodio::queue::queue(true);
    let _ = stream_handle.play_raw(queue_rx);

    while let Ok(msg) = player_stream.player_req().recv() {
        match msg {
            PlayerRequest::Play(song) => {
                match player_stream.play(&queue_tx, song.file_path()) {
                    Ok(()) => player_stream.player_res().send(Ok(())),
                    Err(e) => player_stream.player_res().send(Err(e)),
                };
            }
            PlayerRequest::Pause => {
                player_stream.pause();
                player_stream.player_res().send(Ok(()));
            }
            PlayerRequest::Resume => {
                player_stream.resume();
                player_stream.player_res().send(Ok(()));
            }
            PlayerRequest::SetVolume(volume) => {
                player_stream.set_volume(volume);
                player_stream.player_res().send(Ok(()));
            }
            s => {
                eprintln!("Not implemented '{:?}'", s);
            }
        }
    }
    Ok(())
}

pub fn get_default_output_device(host_id: cpal::HostId) -> cpal::Device {
    eprintln!("Available audio systems:");
    for host in cpal::available_hosts() {
        eprintln!("host: {:?}", host);
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
