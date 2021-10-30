# Configuration
Dizi reads configurations from the following directories using environment variables (in order of precedence):
 - `$DIZI_CONFIG_HOME`
 - `$XDG_CONFIG_HOME/dizi`
 - `$HOME/.config/dizi`

Dizi can currently be configured using the following files:

## Client Configuration
- [client.toml](/docs/configuration/client.toml.md): configuring the client
- [keymap.toml](/docs/configuration/keymap.toml.md): configuring the client's keymapping
- [layout.json](/docs/configuration/layout.json.md): configuring the look of client
- [theme.toml](/docs/configuration/theme.toml.md): theming configurations

## Server Configuration
- [server.toml](/docs/configuration/server.toml.md): configuring the server
