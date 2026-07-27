pub fn get_default_host(host_id: cpal::HostId) -> cpal::Host {
    tracing::debug!("Available audio systems:");
    for host in cpal::available_hosts() {
        tracing::debug!(?host, "Audio host");
    }
    cpal::available_hosts()
        .into_iter()
        .find(|id| *id == host_id)
        .and_then(|id| cpal::host_from_id(id).ok())
        .unwrap_or_else(cpal::default_host)
}
