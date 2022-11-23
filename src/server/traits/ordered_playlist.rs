use dizi_lib::song::Song;

pub struct OrderedPlaylistEntry {
    pub song_index: usize,
    pub playlist_index: usize,
    pub entry: Song,
}

pub trait OrderedPlaylist {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn push(&mut self, song: Song);
    fn remove(&mut self, index: usize);
    fn clear(&mut self);
    fn iter(&self) -> std::slice::Iter<'_, Song>;
    fn swap(&mut self, index1: usize, index2: usize);

    fn is_end(&self) -> bool;

    fn entry_ref(&self, index: usize) -> &Song;
    fn entry_mut(&mut self, index: usize) -> &mut Song;

    fn current_entry_details(&self) -> Option<OrderedPlaylistEntry>;

    fn next_song_peak(&self) -> Option<OrderedPlaylistEntry>;
    fn previous_song_peak(&self) -> Option<OrderedPlaylistEntry>;

    fn next_song(&mut self) -> Option<OrderedPlaylistEntry>;
    fn previous_song(&mut self) -> Option<OrderedPlaylistEntry>;
}
