# server.toml

This file is for configuring the server

```toml
[server]
socket = "/tmp/dizi-server-socket"

# Where to save playlist on exit
playlist = "~/.config/dizi/playlist.m3u"

# not implemented
poll_rate = 200

# run a script whenever the song changes
# on_song_change = "some_script"

[server.player]
# supports alsa, jack
audio_system = "alsa"

shuffle = false
repeat = true
next = true
```
