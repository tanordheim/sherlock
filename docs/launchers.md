# Launchers

In the Sherlock Application Launcher, each tile is associated with a specific "launcher." You can think of a launcher as a category to which tiles belong. For example, if a launcher is set to invisible, all tiles under that launcher will also be invisible.<br>

Launchers are defined in the `fallback.json` file located in your config directory (`/home/user/.config/sherlock/`). If the application cannot find your configuration, it will fallback to the default configuration, which is stored in [resources/fallback.json](https://github.com/Skxxtz/sherlock/blob/unstable/release-v0.1.10/resources/fallback.json).<br>

> **Example File:** [fallback.json](https://github.com/Skxxtz/sherlock/blob/main/docs/examples/fallback.json)
<br>

The launcher can be of the following types:<br>

- **[Category Launcher](#category-launcher):** Groups your launchers.
- **[App Launcher](#app-launcher):** Launches your apps. 
- **[Web Launcher](#web-launcher):** Opens the ``{keyword}`` in your default web browser. The used search engine is configurable and the most common search engines are included. 
- **[Calculator](#calculator):** Converts your input into a math equation and displays its result. On Return, it also copies the result into the clipboard.
- **[Clipboard Launcher](#clipboard-launcher):** Checks if your clipboard currently holds a URL. On Return, it opens the URL in the default web browser. Also displays hex and rgb colors.
- **[Command](#command-launcher):** This field can execute commands that do not rely on the ``{keyword}`` attribute (such as connecting to a specific wifi).
- **[Bulk Text](#bulk-text):** The Bulk Text is a way to launch a custom script/application in an async form and to display its result in a widget.
- **[Teams Event Launcher](#teams-event):** This launcher is capable of joining Microsoft Teams meetings that are scheduled to begin between 5mins ago and in 15mins. 
- **[Music Player Launcher](#music-player):** This launcher shows the currently playing song with artist and toggles playback on return.
- **[Process Terminator](#process-terminator):** This utility shows user processes and terminates them on return.
- **[Weather Launcher](#weather-launcher):** It shows the current weather condition in your selected region or city.

## Shared Launcher Attributes
`[UI]` - used for UI <br>
`[FC]` - used to specify behaviour <br>
| Attribute   | Type | Description |
|-------------|------|-------------|
| `name`      | `[UI]` (optional) | The name of the category the tiles belong to. This name will appear under the app’s name. It is required but can be left empty. |
| `alias`     | `[FC]` (optional) | The command used to search within this category. |
| `home`      | `[FC]` (optional) | Determines if the elements of this launcher are displayed at startup. |
| `only_home`      | `[FC]` (optional) | Determines if the launcher should be included in searches or only be shown on startup. |
| `type`      | `[FC]` (required) | Specifies the tile and functionality to be used for this Launcher. |
| `args`      | `[FC]` (required) | Arguments specific to the `type`. Can be left empty. |
| `priority`  | `[FC]` (required) | Defines the display order of launcher elements at startup. **A value of 0 means the launcher will only be shown if the `alias` is active.**|
| `async`     | `[FC]` (optional) | Indicates whether the launcher should run asynchronously. This is used in `Bulk Text`. |
| `on_return`     | `[FC]` (optional) | Specifies what to do if return is pressed on the tile. |
| `spawn_focus`     | `[FC]` (optional) | Determines whether the tile should automatically gain focus when it appears as the first item in the list. |
| `shortcut`     | `[FC]` (optional) | Determines whether the tile should have the shortcut indicator on the side. |

---

## Category Launcher
```json
{
    "name": "Categories",
    "alias": "cat",
    "type": "categories",
    "args": {
        "Kill Processes": {
            "icon": "sherlock-process", 
            "icon_class": "reactive", 
            "exec": "kill", 
            "search_string": "terminate;kill;process"
            },
        "Power Menu": {
            "icon": "battery-full-symbolic",
            "icon_class": "reactive", 
            "exec": "pm", 
            "search_string": "powermenu;"
            }
    },
    "priority": 3,
    "home": true
}

```
### Arguments (args):
**commands**:<br>
(required)<br>
1. `name field` / the name you want to have displayed for the category
2. `icon` / the icon-name for the icon to display 
3. `exec` / the alias of the launcher you want to execute

<br>
(optional)<br>
1. `icon_class` / Sets the css class for the icon to style it according to your theme
2. `search_string` / the string to match to on search

<br>
---

## App Launcher
```json
{
    "name": "App Launcher",
    "alias": "app",
    "type": "app_launcher",
    "args": {},
    "priority": 2,
    "home": true
}
```
---
## Web Launcher
```json
{
    "name": "Web Search",
    "display_name": "Google Search"
    "tag_start": "{keyword}",
    "tag_end": "{keyword}",
    "alias": "gg",
    "type": "web_launcher",
    "args": {
        "search_engine": "google",
        "icon": "google"
    },
    "priority": 100
}
```
### Arguments (args):
**`search_engine`** (required):
Can be either of the following:
| Search Engine   | URL                                      |
|-----------------|------------------------------------------|
| **Google**      | `https://www.google.com/search?q={keyword}` |
| **Bing**        | `https://www.bing.com/search?q={keyword}` |
| **DuckDuckGo**  | `https://duckduckgo.com/?q={keyword}`    |
| **Yahoo**       | `https://search.yahoo.com/search?p={keyword}` |
| **Baidu**       | `https://www.baidu.com/s?wd={keyword}`   |
| **Yandex**      | `https://yandex.com/search/?text={keyword}` |
| **Ask**         | `https://www.ask.com/web?q={keyword}`    |
| **Ecosia**      | `https://www.ecosia.org/search?q={keyword}` |
| **Qwant**       | `https://www.qwant.com/?q={keyword}`     |
| **Startpage**   | `https://www.startpage.com/sp/search?q={keyword}` |
| **Custom**      | `https://www.example.com/search={keyword}` |

**`icon`** (required):<br>
Sets the icon-name the launcher should show. For a guide on how to add your own icons see [!WARNING]

---

## Calculator
```json
{
    "name": "Calculator",
    "type": "calculation",
    "args": {
        "capabilities": [
            "calc.math",
            "calc.units"
        ]
    },
    "priority": 1,
}
```
### Arguments (args):
**`capabilities`** (optional):<br>
Specifies what the launcher should parse: 
- **`calc.math`** - displays the solutions to mathematical problems 
- **`calc.lengths`** - displays the solutions to length transformations 
- **`calc.weights`** - displays the solutions to weight transformations 
- **`calc.volumes`** - displays the solutions to volume transformations 
- **`calc.temperatures`** - displays the solutions to temerature transformations 
> **💡 Note:** You can also use `calc.units` to use all available unit transformations

---
## Clipboard Launcher
```json
    {
        "name": "Clipboard",
        "type": "clipboard-execution",
        "args": {
            "capabilities": ["url", "hex", "calc.math", "calc.lengths", "calc.weights"]
        },
        "priority": 1,
        "home": true
    }
```
### Arguments (args):
**`capabilities`** (optional):<br>
Specifies what the launcher should parse: 
- **`url`** - parses URLs to launch in the web browser
- **`colors.hex`** - displays hex colors in Sherlock. Format supports #[a-fA-F0-9]{6,8}
- **`colors.rgb`** - displays rgb colors in Sherlock. Format supports optional rgb prefix and optional parentheses.
- **`colors.hsl`** - displays hsl colors in Sherlock. Format supports optional hsl prefix and optional parentheses.
- **`calc.math`** - displays the solutions to mathematical problems 
- **`calc.lengths`** - displays the solutions to length transformations 
- **`calc.weights`** - displays the solutions to weight transformations 
- **`calc.volumes`** - displays the solutions to volume transformations 
- **`calc.temperatures`** - displays the solutions to temerature transformations 
> **💡 Note:** 
> You can also use
> - `colors.all` to use all available color formats
> - `calc.units` to use all available unit transformations

---
## Command Launcher
```json
{
    "name": "Example Command",
    "alias": "ex",
    "type": "command",
    "args": {
        "commands": {
            "command name": {
                "icon": "icon-name",
                "icon_class": "reactive",
                "exec": "command to execute", 
                "search_string": "examplecommand"
                "tag_start": "{keyword}"
                "tag_end": "{keyword}"
            },
            "command2": {
                "icon": "icon-name",
                "exec": "command to execute", 
                "search_string": "examplecommand"
                "tag_start": "{keyword}"
                "tag_end": "{keyword}"
            }
        }
    },
    "priority": 5
}
```
### Arguments (args):
**commands** (required):<br>
Has following fields of its own:
1. `name field` / the name of the application (is the field where command name is the value currently)
2. `icon` / the icon-name for the icon to display 
3. `icon_class` / Sets the css class for the icon to style it according to your theme
4. `exec` / the command to execute
5. `search_string` / the string to match to on search
6. `tag_start` / specifies what will be displayed in the start tag
7. `tag_end` / specifies what will be displayed in the end tag

---

## Bulk Text
```json
{
    "name": "Wikipedia Search",
    "alias": "wiki",
    "type": "bulk_text",
    "async": true,
    "on_return": "copy",
    "args": {
        "icon": "wikipedia",
        "exec": "~/.config/sherlock/scripts/sherlock-wiki"
        "exec-args": "{keyword}"
    },
    "priority": 0,
    "shortcut": false
}
```
### Arguments (args):
**`icon`** (required):<br>
Specifies the icon shown for the command.<br>

**`exec`** (required):<br>
Specifies the program that should be run. **Note:** that its probably suitable to run it asynchronously. To do that, set the `async` attribute to `true`. <br>

**`exec-args`** (optional):<br>
Specifies the arguments to pass along to the `exec` program.<br>

> The provided snippet works with the project [sherlock-wiki](https://github.com/Skxxtz/sherlock-wiki) 
--- 

## Teams Event
> **🚨 Warning:** Currently only supports Thunderbird calendar events
```json
{
    "name": "Teams Event",
    "type": "teams_event",
    "args": {
        "event_date": "now",
        "event_start": "-5 minutes",
        "event_end": "+15 minutes"
    },
    "priority": 1,
    "home": true
},
```

### Arguments (args):
**`icon`** (optional):<br>
Specifies the icon shown for the event.<br>

**`event_date`** (optional):<br>
Specifies the date for the event lookup<br>

**`event_start`** (optional):<br>
Specifies the offset from the `date` parameter.<br>

**`event_end`** (optional):<br>
Specifies the second offset from the `date` parameter.<br>

--- 

## Music Player
```json
{
    "name": "Spotify",
    "type": "audio_sink",
    "args": { },
    "async": true,
    "priority": 1,
    "home": true,
    "only_home": true,
    "spawn_focus": false
},

```

### Arguments (args):
None

--- 

## Process Terminator
```json
{
    "name": "Kill Process",
    "alias": "kill",
    "type": "process",
    "args": { },
    "priority": 6,
    "home": false
},

```

### Arguments (args):
None

--- 

## Weather Launcher
```json
{
    "name": "Weather",
    "type": "weather",
    "args": {
        "location": "berlin",
        "update_interval": 60
    },
    "priority": 1,
    "home": true,
    "only_home": true,
    "async": true,
    "shortcut": false,
    "spawn_focus": false
},


```

### Arguments (args):

**`location`** (required):<br>
Specifies the location for which the weather should be gathered.<br>

**`update_interval`** (optional):<br>
Specifies how often you want to update the weather. In minutes.<br>

--- 
