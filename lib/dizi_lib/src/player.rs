#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlaylistStatus {
    DirectoryListing,
    PlaylistFile,
}
