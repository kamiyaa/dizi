use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use log::{debug, log_enabled, Level};

pub fn get_default_host(host_id: cpal::HostId) -> cpal::Host {
    if log_enabled!(Level::Debug) {
        debug!("Available audio systems:");
        for host in cpal::available_hosts() {
            debug!("host: {:?}", host);
        }
    }
    cpal::host_from_id(
        cpal::available_hosts()
            .into_iter()
            .find(|id| *id == host_id)
            .unwrap(),
    )
    .unwrap_or_else(|_| cpal::default_host())
}

pub fn play_stream(device: &cpal::Device) {
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let stream = device
        .build_output_stream(&config, write_silence::<f32>, err_fn)
        .unwrap();

    stream.play().unwrap();
}

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&1.0);
    }
}
