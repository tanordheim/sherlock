use std::path::Path;
use std::{fs, env};

use super::Loader;
use super::util::{Config, get_terminal};


impl Loader {
    pub fn load_config() -> Result<Config, String> {
        let default_config = Config::default();
        let home_dir = env::var("HOME").map_err(|e| format!("Cannot unpack home directory for user. Error: {}", e))?;
        let user_config_path = format!("{}/.config/sherlock/config.toml", home_dir);

        // Attempt to read and parse the user config file if it exists
        let user_config: Config = if Path::new(&user_config_path).exists() {
            let config_str = fs::read_to_string(&user_config_path)
                .map_err(|e| format!("Failed to read the user config file. Error: {}", e))?;

            toml::de::from_str(&config_str)
                .map_err(|e| format!("Could not parse user config. Error: {}", e))?
        } else {
            default_config
        };

        // Ensure terminal configuration is set
        let mut final_config = user_config;
        if final_config.default_apps.terminal.is_none() {
            final_config.default_apps.terminal = get_terminal();
        }

        Ok(final_config)
    }
}

