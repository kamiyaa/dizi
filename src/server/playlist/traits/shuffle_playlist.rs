pub trait ShufflePlaylist {
    fn shuffle_enabled(&self) -> bool;
    fn set_shuffle(&mut self, shuffle: bool);

    fn shuffle(&mut self);
    fn unshuffle(&mut self);
}
