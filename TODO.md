## UI
- [x] Improve startup time more by reducing amount of widgets shown in the beginning. Let user configure the launchers shown on startup.

## Scripts
- [ ] Custom Spotify (ncspot) script to control it

## Functionalities
- [x] Implement locking mechanism to only have one instance running at a time.
- [ ] Implement a next() function that adds a layout to the launcher stack.
- [ ] → Make a widget that uses gtk4::Builder::from_string(ui_string)
- [ ] Make more widgets async-able 
- [ ] Maybe change alias for each command in commandlauncher. (if possible)
- [ ] Add ArgCommand launcher type or convert existsing one to be more versatile
- [ ] Add callback type for a cmd to execute another one
- [ ] Add enter command type for a tile
- [ ] Implement command execution count and sort commands based on that
- [ ] Finish the setup of the loading animation for async widget(s)

## Configuration
- [x] Let user set flags such as --config to set the fallbacks-, style-, and config-files
- [x] Implement custom file for aliases and custom icons for apps.
- [x] Improve sherlock ignore file to use * macro and case sensitivity
- [ ] Better user integration from custom scripts. i.e. give control over tiles to spawn

