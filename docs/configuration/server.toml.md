# server.toml

This file is for configuring the server

```toml
[server]
# socket path for clients to connect to
socket = "/tmp/dizi-server-socket"

# Where to save playlist on exit
playlist = "~/.config/dizi/playlist.m3u"

# How often to poll audio thread for updates in milliseconds (not implemented)
# slower = less responsive player
# faster = more cpu usage (from busy waiting)
poll_rate = 200

# path to run a script whenever the song changes
# on_song_change = "some_script"

[server.player]
# supports alsa, jack on Linux
# will use the default on other systems (MacOS, Windows)
audio_system = "alsa"

shuffle = false
repeat = true
next = true
```
