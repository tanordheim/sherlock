# Sherlock Application Launcher
Sherlock is a lightweight and efficient application launcher built with Rust and GTK4. It allows you to quickly launch your favorite applications with a user-friendly interface, providing a fast and highly-configurable way to search, launch, and track application usage.


## Dependencies
- gtk4
- gtk-4-layer-shell


## Launchers
- **App Launcher:** Launches your apps. 
- **Web Launcher:** Opens the ``{keyword}`` in your default webbrowser. The used search engine is configureable and the most common search engines are included. 
- **Calculator:** Converts your input into a math equation and displays its result. On Enter, it also copies the result into the clipboard.
- **Command:** This field can execute commands that do not rely on the ``{keyword}`` attribute (such as connecting to a specific wifi).
- **Bulk Text:** The Bulk Text is a way to launch a custom script/application in an async form and to display its result in a widget.

### Common Launcher Argument
`[UI]` corresponds to ui related attributes.\n
`[FC]` corresponds to functionality related attributes.

- `name` [UI] (required): Specifies the name of the category the resulting tiles corresponds to. This name will be displayed under the apps name. It has no further impact on the application but **must be set but can be empty**. 
- `alias` [FC] (optional): Specifies what the command should be to search that category with.
- `home` [FC] (optional): Defines, wheather the elements of this launcher should be shown on startup.
- `type` [FC] (required): Specifies the tile and functionality that should be used to display this Launcher.
- `args` [FC] (required): A value with `type` specific arguments. **Can be empty**.
- `priority` [FC] (required): Specifies the order in which to show the launcher elements on startup. 

