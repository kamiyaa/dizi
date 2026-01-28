pub fn get_default_host(host_id: cpal::HostId) -> cpal::Host {
    tracing::debug!("Available audio systems:");
    for host in cpal::available_hosts() {
        tracing::debug!(?host, "Audio host");
    }
    cpal::host_from_id(
        cpal::available_hosts()
            .into_iter()
            .find(|id| *id == host_id)
            .unwrap(),
    )
    .unwrap_or_else(|_| cpal::default_host())
}
