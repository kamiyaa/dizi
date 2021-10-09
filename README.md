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

 - [x] play/pause support
 - [ ] playlist support
 - [ ] show music progress
 - [ ] shuffle, repeat, next
 - [ ] volume support
 - [ ] theming support
 - [ ] Pipewire to play audio
 - [ ] custom layout support
