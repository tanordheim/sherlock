# Sherlock Application Launcher
Sherlock is a lightweight and efficient application launcher built with Rust and GTK4. It allows you to quickly launch your favorite applications with a user-friendly interface, providing a fast and highly-configurable way to search, launch, and track application usage.


## Dependencies
- gtk4
- gtk-4-layer-shell


## Launchers
- **[App Launcher](#app-launcher):** Launches your apps. 
- **[Web Launcher](#web-launcher):** Opens the ``{keyword}`` in your default webbrowser. The used search engine is configureable and the most common search engines are included. 
- **[Calculator](#calculator):** Converts your input into a math equation and displays its result. On Enter, it also copies the result into the clipboard.
- **[Command](#command-launcher):** This field can execute commands that do not rely on the ``{keyword}`` attribute (such as connecting to a specific wifi).
- **[Bulk Text](#bulk-text):** The Bulk Text is a way to launch a custom script/application in an async form and to display its result in a widget.



### Common Launcher Attributes
`[UI]` corresponds to ui related attributes.<br>
`[FC]` corresponds to functionality related attributes.<br>

- `name` `[UI]` (required): Specifies the name of the category the resulting tiles corresponds to. This name will be displayed under the apps name. It has no further impact on the application but **must be set but can be empty**. 
- `alias` `[FC]` (optional): Specifies what the command should be to search that category with.
- `home` `[FC]` (optional): Defines, wheather the elements of this launcher should be shown on startup.
- `type` `[FC]` (required): Specifies the tile and functionality that should be used to display this Launcher.
- `args` `[FC]` (required): A value with `type` specific arguments. **Can be empty**.
- `priority` `[FC]` (required): Specifies the order in which to show the launcher elements on startup. 
- `async` `[FC]` (optional): Specifies if the launcher should be executed asynchronously. Implemented for `Bulk Text`
---

### App Launcher
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
### Web Launcher
```json
{
    "name": "Web Search",
    "alias": "gg",
    "type": "web_launcher",
    "args": {
        "search_engine": "google",
        "icon": "google"
    },
    "priority": 100
}
```
#### Arguments (args):
**seach_engine** (required):<br>
Can be either of those:
1. google -> "https://www.google.com/search?q={keyword}"
2. bing -> "https://www.bing.com/search?q={keyword}"
3. duckduckgo -> "https://duckduckgo.com/?q={keyword}"
4. yahoo -> "https://search.yahoo.com/search?p={keyword}"
5. baidu -> "https://www.baidu.com/s?wd={keyword}"
6. yande -> "https://yandex.com/search/?text={keyword}"
7. ask -> "https://www.ask.com/web?q={keyword}"
8. ecosia -> "https://www.ecosia.org/search?q={keyword}"
9. qwant -> "https://www.qwant.com/?q={keyword}"
10. startpage -> "https://www.startpage.com/sp/search?q={keyword}"
11. custom -> "https://www.example.com/search={keyword}"
<br>
**icon** (required):<br>
Sets the icon-name the launcher should show. For a guide on how to add your own icons see [!WARNING]

---

### Calculator
```json
{
    "name": "Calculator",
    "type": "calculation",
    "args": {},
    "priority": 1,
}
```

---

### Command Launcher
```json
{
    "name": "Example Command",
    "alias": "ex",
    "type": "command",
    "args": {
        "commands": {
            "command name": {
                "icon": "icon-name",
                "exec": "command to execute", 
                "search_string": "examplecommand"
            }
        }
    },
    "priority": 5
}
```
#### Arguments (args):
**commands** (required):<br>
Has following fields of its own:
1. name field / the name of the applicaiton
2. icon / the icon-name for the icon to display 
3. exec / the command to execute
4. search_string / the string to match to on search

---

### Bulk Text
```json
{
    "name": "Wikipedia Search",
    "alias": "wiki",
    "type": "bulk_text",
    "async": true,
    "args": {
        "icon": "wikipedia",
        "exec": "wiki-api"
        "exec-args": "{keyword}"
    },
    "priority": 5
}
```
#### Arguments (args):
**icon** (required):<br>
Specifies the icon shown for the command.<br>

**exec** (required):<br>
Specifies the program that should be run. **Note:** that its probably suitable to run it asynchronously. To do that, set the `async` attribute to `true`.

**exec-args** (optional):<br>
Specifies the arguments to pass along to the `exec` program.

--- 
