[![Linux build](https://github.com/kamiyaa/dizi/actions/workflows/rust-linux-main.yml/badge.svg)](https://github.com/kamiyaa/dizi/actions/workflows/rust-linux-main.yml)

[![MacOS build](https://github.com/kamiyaa/dizi/actions/workflows/rust-macos-main.yml/badge.svg)](https://github.com/kamiyaa/dizi/actions/workflows/rust-macos-main.yml)

# dizi
Server-client music player written in Rust

The goal of this project is to create a modern version of [mocp](http://moc.daper.net/) in Rust.

![Alt text](screenshot.png?raw=true "dizi")

## Motivation
mocp currently interfaces with ALSA to play audio.
This doesn't play well with [pipewire](https://pipewire.org/)'s pipewire-alsa plugin;
whenever mocp is playing music, other audio/video apps stop working and vice versa.

## Dependencies
 - A system supporting UNIX sockets
 - [cargo](https://github.com/rust-lang/cargo/)
 - [rustc](https://www.rust-lang.org/)
 - Jack or Alsa or any other audio system [cpal](https://github.com/RustAudio/cpal) supports
 - `file` command for audio file detection

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
~ $ dizi-server     # starts server
~ $ RUST_LOG=debug dizi-server      # starts server with debug messages enabled
~ $ dizi            # starts server if not already started, then starts frontend
```

## Configuration

Check out [docs](/docs) for details and [config](/config) for examples

#### [client.toml](/config/client.toml)
- client configurations

#### [keymap.toml](/config/keymap.toml)
- for keybindings for client

#### [theme.toml](/config/theme.toml)
- color customizations for client

#### [server.toml](/config/server.toml)
- server configurations

## Contributing
See [docs](/docs)

## Features/Bugs

Please create an issue :)

## TODOs

### Server-side
 - [x] play/pause support
 - [x] get audio duration (requires [rodio](https://github.com/RustAudio/rodio) and [symphonia](https://github.com/pdeljanov/Symphonia) to work together on this)
 - [x] volume support
 - [x] fast forward/rewind
 - [x] directory playing
   - [x] shuffle
   - [x] repeat
   - [x] next
   - [ ] sorting
 - [x] playlist support
   - [x] add/delete/update songs
   - [x] recursively add songs in a directory
   - [x] shuffle
   - [x] repeat
   - [x] next
   - [x] loading
   - [x] clearing
   - [x] save on exit
 - [x] show music progress
 - [x] configurable audio system
   - [x] ALSA support (current default)
   - [x] JACK support
   - [ ] Pulseaudio support (issue https://github.com/RustAudio/cpal/issues/259)
   - [ ] Pipewire support (issue https://github.com/RustAudio/cpal/issues/554)
 - [x] querying
   - [x] file name
   - [x] file path
   - [x] show audio metadata (title, artists, genre, album, etc)
   - [x] playlist index and length
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
 - [ ] show audio metadata (artists, genre, album, etc)
 - [x] theming support
 - [x] custom layout support
