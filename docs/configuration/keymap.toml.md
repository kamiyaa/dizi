# keymap.toml
This file is for mapping keyboard keys to commands.

```toml
[[keymap]]
keys = [ "arrow_up" ]
command = "cursor_move_up"

[[keymap]]
keys = [ "9" ]
command = "server_request"
json.request = "/player/volume/decrease"
json.amount = 1
```

# Keys available:

To combine keys with Ctrl and Alt, simply have `ctrl+key`/`alt+key`
where `key` is a valid key.

In addition to the standard alphabet, the following keys are also supported:
```sh
backspace
backtab     # this is shift+tab
arrow_left
arrow_right
arrow_up
arrow_down
home
end
page_up
page_down
delete
insert
escape
f1
f2
f3
f4
f5
f6
f7
f8
f9
f10
f11
f12
```

# Commands available:

- [Client](#Client)
- [Server Requests](#Server Requests)

## Client

## Server Requests
