use std::path::Path;
use std::{fs, env};

use super::Loader;
use super::util::{Config, get_terminal};

impl Loader {
    pub fn load_config() -> Config {
        let default_config = Config::default();

        let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home/user"));
        let user_config_path = format!("{}/.config/sherlock/config.toml", home_dir);

        // Check if the user has a custom config file
        let mut user_config = if Path::new(&user_config_path).exists() {
            let config_str = fs::read_to_string(&user_config_path)
                .unwrap_or_else(|_| {
                    println!("Error reading config file, using defaults.");
                    String::new()
                });

            // Try to deserialize the user configuration, falling back to defaults
            toml::de::from_str(&config_str)
                .unwrap_or_else(|e| {
                    println!("Failed to deserialize config, using defaults. {}", e);
                    default_config
                })
        } else {
            default_config
        };

        if user_config.default_apps.terminal.is_none() {
            user_config.default_apps.terminal = get_terminal();
        };
        user_config
    }
}
