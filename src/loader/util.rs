use std::fs::File;
use std::io::{BufReader, Read};
use std::env;
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CommandConfig {
    pub name: String,
    pub alias: Option<String>,
    pub r#type: String,
    pub priority: u32,

    #[serde(default)]
    pub r#async: bool,
    #[serde(default)]
    pub home: bool,
    #[serde(default)]
    pub args: serde_json::Value,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppData{
    pub icon: String,
    pub exec: String,
    pub search_string: String,
}

#[derive(Debug)]
pub struct SherlockFlags {
    pub config: String,
    pub fallback: String,
    pub style: String,
    pub ignore: String,
    pub alias: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SherlockAlias{
    pub name: Option<String>,
    pub icon: Option<String>,
    pub keywords: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SherlockError {
    pub name: String,
    pub message: String,
    pub traceback: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Config{
    #[serde(default)]
    pub default_apps: ConfigDefaultApps,
    #[serde(default)]
    pub appearance: ConfigAppearance,
}
impl Config {
    pub fn default()->Self{
        Config {
            default_apps: ConfigDefaultApps {
                terminal: get_terminal(),
            },
            appearance: ConfigAppearance {
                gsk_renderer: "cairo".to_string(),
                recolor_icons: false,
                icon_paths: Default::default(),
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
    pub struct ConfigDefaultApps{
        #[serde(default)]
        pub terminal: Option<String>,
    }
#[derive(Deserialize, Debug, Clone, Default)]
    pub struct ConfigAppearance{
        #[serde(default)]
        pub gsk_renderer: String, 
        #[serde(default)]
        pub recolor_icons: bool,
        #[serde(default)]
        pub icon_paths: Vec<String>, 
    }

    pub fn read_file(file_path: &str)->std::io::Result<String>{
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
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

