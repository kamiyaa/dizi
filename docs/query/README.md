# Client

## Queries

You can query the server for information via

```sh
~$ dizi -q 'QUERY'
```

Available query strings include:

```
player_status   # playing, paused, stopped
player_volume   # between 0 and 100
player_next     # boolean (true, false) if go to next song is enabled
player_repeat   # boolean (true, false) if repeat is enabled
player_shuffle  # boolean (true, false) if shuffle is enabled
file_name       # file name of current song
file_path       # file path of current song
playlist_status # (file, directory) whether player is
                # playing a playlist file or from a directory

playlist_index  # index of the song being played in the file playlist
playlist_length # length of playlist
