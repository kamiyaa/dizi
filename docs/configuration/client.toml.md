# client.toml

This file is for configuring the client

```toml
[client]
# socket path for connecting to server
socket = "/tmp/dizi-server-socket"

# the directory to start the client in
home_dir = "~/music"

[client.display]
# show borders around widgets
show_borders = true

# show hidden files
show_hidden = false

# layout file
layout = "~/.config/dizi/layout.json"

[client.display.sort]
# list directory first
directory_first = true
# reverse directory
reverse = false

# Options include
# - lexical  (10.txt comes before 2.txt)
# - natural  (2.txt comes before 10.txt)
# - mtime
sort_method = "natural"
```
