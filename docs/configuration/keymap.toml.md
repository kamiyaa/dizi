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

- [Client](#client)
- [Server Requests](#server-requests)

## Client
`help`: opens help menu

`close`: close the client without quitting the server

`cd`: change directory
 - `cd ..`: go to parent directory
 - `cd ~`: go to home directory

`:`: opens the command prompt
   - this does not execute the command, but merely sets the text to it
   - Example: `:cd /` will open up the command prompt with `cd /` already written

`cursor_move_up`: moves the cursor up by x amount
 - `cursor_move_up`: moves the cursor up by 1
 - `cursor_move_up x`: moves the cursor up by `x` where `x` is a non-negative number

`cursor_move_down`: moves the cursor down by x amount
 - `cursor_move_down`: moves the cursor down by 1
 - `cursor_move_down x`: moves the cursor down by `x` where `x` is a non-negative number

`cursor_move_home`: moves cursor to beginning of directory list

`cursor_move_end`: moves cursor to end of directory list

`cursor_move_page_up`: moves the cursor up by `x`
 - where `x` is the number of items that can be seen on the screen

`cursor_move_page_down`: moves the cursor down by `x`
 - where `x` is the number of items that can be seen on the screen

`open`: play music file or open directory

`reload_dirlist`: reloads the current directory listing

`search`: search via string
 - case insensitive

`search_glob`: search via shell globbing
 - `:search_glob *.png`

`search_skim`: search via fzf

`search_next`: go to next search result

`search_prev`: go to previous search result

`sort`: change the sort method
 - `sort lexical`: sort lexically (`10.txt` comes before `2.txt`)
 - `sort natural`: sort naturally (`2.txt` comes before `10.txt`)
 - `sort mtime`: sort via last modified time
 - `sort reverse`: reverse the sorting

`toggle_hidden`: toggle hidden files

`toggle_view`: switch between file browser and playlist widget

## Server Requests
