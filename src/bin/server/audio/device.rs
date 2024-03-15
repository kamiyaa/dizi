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
