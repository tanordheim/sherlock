## UI
- [x] Improve startup time further by reducing the number of widgets shown initially. Allow the user to configure which launchers are displayed at startup.
- [x] Add error tile.

## Scripts
- [ ] Create a custom Spotify (ncspot) script to control it.
- [x] On startup, check if the clipboard contains a URL and enable URL search from it.
    - [] Improve the functionality further for example display colors


## Functionalities
- [x] Implement a locking mechanism to ensure only one instance is running at a time.
- [ ] Implement a `next()` function that adds a layout to the launcher stack to navigate within Sherlock, similar to a follow-up screen.
- [ ] → Create a widget that uses `gtk4::Builder::from_string(ui_string)`.
- [ ] Make more widgets asynchronous.
- [ ] Consider changing the alias for each command in `commandlauncher` (if possible).
- [ ] Add an `ArgCommand` launcher type or convert the existing one to be more versatile.
- [ ] Add a callback type for a command to execute another command.
- [ ] Add an "enter" command type for a tile.
- [ ] Implement command execution count and sort commands based on that count.
- [ ] Finish setting up the loading animation for asynchronous widgets.

## Configuration
- [x] Allow the user to set flags such as `--config` to configure fallback, style, and config files.
- [x] Implement a custom file for aliases and custom icons for apps.
- [ ] Improve Sherlock ignore file to support `*` macro and case sensitivity.
- [ ] Provide better user integration from custom scripts, e.g., give control over tiles to spawn.
- [ ] Make it possible to customize the height and width of the window.

## Documentation
- [ ] Implement Flag Heading in the README.md.

- [ ] Implement tags on command widgets and use custom CSS classes for them.
- [ ] Implement the possibility to customize categories and their UI files. Allow specifying the UI files used for the categories. Required: category config file.

