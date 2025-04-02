# Flags

## Normal Flags
| Flag         | Description | Note |
|--------------|----------------------|------------------------------------------------|
| --version    | Print the version of the application.| None  |
| --help       | Show this help message with allowed flags.                                        | None                                           |
| --config     | Specify the configuration file to load. | Recommended location: `~/.config/sherlock/config.toml` |
| --fallback   | Specify the fallback file to load. | Recommended location: `~/.config/sherlock/fallback.json`             |
| --style      | Set the style configuration file. | Recommended location: `~/.config/sherlock/style.css`                 |
| --ignore     | Specify the Sherlock ignore file. | Recommended location: `~/.config/sherlock/sherlockignore`            |
| --alias      | Specify the Sherlock alias file. |Recommended location: `~/.config/sherlock/sherlock_alias.json`       |
| --cache      | Specify the location for the caching file. Sets caching active |Recommended location: `~/.cache/sherlock_desktop_cache.json`       |
| --daemonize      | Overrides the deamonizing value in `config.toml` | |

## Pipe Mode Flags
| Flag         | Description | Note |
|--------------|----------------------|------------------------------------------------|
| --display-raw      | Displays the piped input as a text field. Useful for term graphics. | |
| --center | Centers the content.  | Only works with `--display-raw`|
| --method | Specifies the method Sherlock will use to handle return presses. | |

