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
 - Jack or Alsa or any other audio system [cpal](https://github.com/RustAudio/cpal) supports

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
 - [ ] get audio duration (requires [rodio](https://github.com/RustAudio/rodio) and [symphonia](https://github.com/pdeljanov/Symphonia) to work together on this)
 - [x] volume support
 - [x] directory playing
   - [x] shuffle
   - [x] repeat
   - [x] next
   - [ ] sorting
 - [x] playlist support
   - [x] add/delete/update songs
   - [ ] shuffle
   - [x] repeat
   - [x] next
   - [x] loading
   - [x] clearing
   - [x] save on exit
 - [x] show music progress
 - [x] configurable audio system
   - [x] ALSA support
   - [x] JACK support (current default)
   - [ ] Pipewire support (requires [cpal](https://github.com/RustAudio/cpal) to support it)
 - [x] querying
   - [x] file name
   - [x] file path
   - [ ] song name (blocked on metadata)
 - [x] on song change hook

### Client-side
 - [x] show hidden files
 - [x] searching
   - [x] glob search
   - [x] case-insensitive search
   - [x] skim search (fzf)
 - [x] show player progression
 - [x] playlist support
   - [x] show playlist
   - [x] add/delete/update songs
   - [x] shuffle
   - [x] repeat
   - [x] next
   - [x] clearing
 - [ ] show audio metadata
 - [x] theming support
 - [x] custom layout support
