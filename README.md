# dizi
Server-client music player written in Rust (WIP)

The goal of this project is to create a modern version of mocp in Rust.

## Why?
mocp currently interfaces with ALSA to play audio.
This doesn't play well with pipewire-alsa plugin;
whenever mocp is playing music, other audio/video apps stop working and vice versa.

## Dependencies
 - [cargo](https://github.com/rust-lang/cargo/)
 - [rustc](https://www.rust-lang.org/)

## Building
```
~$ cargo build
```

## Installation
#### For single user
```
~$ cargo install --path=. --force
```

#### System wide
```
~# cargo install --path=. --force --root=/usr/local     # /usr also works
```

## Usage
```
~ $ dizi
```

## TODOs

### Server-side
 - [x] play/pause support
 - [x] volume support
 - [x] directory playing
   - [x] shuffle
   - [x] repeat
   - [x] next
 - [ ] playlist support
   - [ ] add/delete/update songs
   - [ ] shuffle
   - [ ] repeat
   - [ ] next
 - [x] show music progress
 - [x] configurable audio system
   - [x] ALSA support
   - [x] JACK support (current default)
   - [ ] Pipewire support

### Client-side
 - [x] show hidden files
 - [x] searching
   - [x] glob search
   - [x] case-insensitive search
   - [x] skim search (fzf)
 - [x] show player progression
 - [ ] playlist support
   - [x] show playlist
   - [ ] add/delete/update songs
   - [ ] shuffle
   - [ ] repeat
   - [ ] next
 - [ ] show audio metadata
 - [ ] theming support
 - [x] custom layout support
