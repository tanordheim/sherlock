use std::{fs, env};
use std::process::Command;
use std::path::Path;

use gtk4::subclass::window;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config{
    pub defaults: Defaults,
}

#[derive(Deserialize, Debug)]
pub struct Defaults{
    pub terminal: Option<String>,
}


pub fn read_config()->Config{
    let mut config = Config{
        defaults :  Defaults{
            terminal : None,
        }
    };
    let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home/user"));
    let user_specific = format!("{}/.config/sherlock/config.toml", home_dir);

    //Check if user has created config file
    if Path::new(&user_specific).exists(){
        let config_str = fs::read_to_string(user_specific).expect("Error reading config file.");
        let user_config: Config = toml::de::from_str(&config_str).expect("Failed to deserialize configs.");

        config = user_config
    }

    if config.defaults.terminal.is_none(){
        config.defaults.terminal = get_terminal();
    }
    return config
}



pub fn get_terminal()->Option<String>{
    let mut terminal = None;

    //Check if $TERMAINAL is set
    if let Ok(term) = env::var("TERMINAL") {
        if is_terminal_installed(&term) {
            terminal = Some(term); 
        }
    }
    // Try other terminals
    if terminal.is_none(){
        let terminals = [
            "kitty", "gnome-terminal", "xterm", "konsole", "alacritty",
            "urxvt", "mate-terminal", "terminator", "sakura", "terminology",
            "st", "xfce4-terminal", "guake", "x11-terminal", "macos-terminal",
            "iterm2", "lxterminal", "foot", "wezterm", "tilix"
        ];
        for t in terminals{
            if is_terminal_installed(t){
                terminal = Some(t.to_string());
                break;
            }
        }

    }

    terminal
}



fn is_terminal_installed(terminal: &str) -> bool {
    Command::new(terminal)
        .arg("--version") // You can adjust this if the terminal doesn't have a "--version" flag
        .output()
        .is_ok()
}
