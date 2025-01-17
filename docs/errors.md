# Terminal Apps

Sherlock supports launching apps that specify `Terminal=True` in their `.desktop` file by using a terminal emulator. The process for determining which terminal to use is as follows:

1. **Configuration Files:** Sherlock first checks the config file `~/.config/sherlock/config.toml` for the `[default] terminal` key. 
2. **Environment Variable:** If the key is not set, Sherlock proceeds to checking the `$TERMINAL` environment variable. This can be set in your shell configuration file (e.g., `.bashrc` or `.zshrc`).
3. **Fallback Options:** As a final fallback, Sherlock attempts to launch one of several commonly used terminal applications. If none are available, it will throw an error: `E5000`.

--- 

## Setting the `$TERMINAL` Environment Variable

To ensure Sherlock uses your preferred terminal emulator, set the `$TERMINAL` environment variable in your shell's configuration file:

1. Open your `.bashrc` (or `.zshrc` for Zsh users) in a text editor:
```bash
nvim ~/.bashrc
```
2. Add the following line, replacing `kitty` with your preferred terminal:
```bash 
export TERMINAL=kitty
```
3. Save the file and reload the configuration:
```bash 
source ~/.bashrc
```
4. Verify the variable is set correctly:
```bash 
echo $TERMINAL
```
The output should display your terminal name, e.g. kitty.

--- 

### Commonly Used Terminal Apps
If no `$TERMINAL` is specified, and the configuration files don't define a terminal, Sherlock will attempt to use one of the following terminal emulators (in order of preference):
## Commonly Used Terminal Applications

- Kitty (`xterm-kitty`)
- GNOME Terminal (`gnome-terminal`)
- Xterm (`xterm`)
- Konsole (`konsole`)
- Alacritty (`alacritty`)
- URxvt (`rxvt-unicode`)
- Mate Terminal (`mate-terminal`)
- Terminator (`terminator`)
- Sakura (`sakura`)
- Terminology (`terminology`)
- St (`st`)
- Xfce4 Terminal (`xfce4-terminal`)
- Guake (`guake`)
- X11 Terminal (`x11-terminal`)
- macOS Terminal (`Terminal`)
- iTerm2 (`iTerm2`)
- LXTerminal (`lxterminal`)
- Foot (`foot`)
- WezTerm (`wezterm`)
- Tilix (`tilix`)

Ensure one of these is installed to avoid the `E5000` error.

---

#### Env Var not Found Error
- Failed to unpack home directory for user.

#### File Read Error
- Failed to read the user configuration file: [file]
- Failed to load provided fallback file: [file]

#### File Parse Error
- Failed to read the user configuration file: [file]
- Failed to parse [file] as a valid UTF-8 string.
- Failed to parse [file] as a valid json.

#### Resource Lookup Error
- Failed to load [file] from resource.

#### Display Error
- Could not connect to a display.
