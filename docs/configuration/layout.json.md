# layout.json

This file is for configuring the look of the client

There are 2 types of widgets:
 - simple
 - composite

`simple`: widgets are standalone widgets.
 - `ratio`: the ratio of how much space a widget takes up in a given composite widget
 - `border`: show borders or not
 - `widget`: currently supports `file_browser`, `music_player`, `playlist`

`composite`: widgets are made up of more widgets.
 - `ratio`: the ratio of how much space a widget takes up in a given composite widget
 - ~~`border`: show borders or not~~
 - `direction`: put widgets beside each other (horizontal) or above/below each other (vertical)
 - `widget`: list of widgets


```json
{
    "layout": {
        "type": "composite",
        "direction": "horizontal",
        "ratio": 1,
        "widgets": [
            {
                "type": "simple",
                "widget": "file_browser",
                "ratio": 1,
                "border": true
            },
            {
                "type": "composite",
                "direction": "vertical",
                "ratio": 1,
                "widgets": [
                    {
                        "type": "simple",
                        "widget": "music_player",
                        "ratio": 2,
                        "border": true
                    },
                    {
                        "type": "simple",
                        "widget": "playlist",
                        "ratio": 3,
                        "border": true
                    }
                ]
            }
        ]
    }
}
```
