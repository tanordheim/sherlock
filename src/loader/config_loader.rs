use std::path::Path;
use std::{fs, env};

use super::Loader;
use super::util::{get_terminal, Config, SherlockError};


impl Loader {
    pub fn load_config() -> Result<Config, SherlockError> {
        let home_dir = env::var("HOME")
                .map_err(|e| SherlockError {
                    name:format!("Env Var not Found Error"),
                    message: format!("Failed to unpack home directory for user."),
                    traceback: e.to_string(),
                })?;
        let user_config_path = format!("{}/.config/sherlock/config.toml", home_dir);

        let user_config = if Path::new(&user_config_path).exists() {
            let config_str = fs::read_to_string(&user_config_path)
                .map_err(|e| SherlockError {
                    name:format!("File Read Error"),
                    message: format!("Failed to read the user configuration file: {}", user_config_path),
                    traceback: e.to_string(),
                })?;

            toml::de::from_str(&config_str)
                .map_err(|e| SherlockError {
                    name:format!("File Parse Error"),
                    message: format!("Failed to parse the user configuration from the file: {}", user_config_path),
                    traceback: e.to_string(),
                })?
        } else {
            Config::default()
        };

        let mut final_config = user_config;
        if final_config.default_apps.terminal.is_none() {
            final_config.default_apps.terminal = get_terminal();
        }

        Ok(final_config)
    }
}

