use std::fs;

use super::util::{Config, SherlockError, SherlockErrorType, SherlockFlags};
use super::Loader;

impl Loader {
    pub fn load_config(
        sherlock_flags: &SherlockFlags,
    ) -> Result<(Config, Vec<SherlockError>), SherlockError> {
        match fs::read_to_string(&sherlock_flags.config) {
            Ok(config_str) => {
                let mut config: Config = match toml::de::from_str(&config_str){
                    Ok(config) => config,
                    Err(e) => {
                        return Err(SherlockError {
                            error: SherlockErrorType::FileParseError(sherlock_flags.config.to_string()),
                            traceback: e.to_string(),
                        })
                    }
                };

                if sherlock_flags.caching {
                    config.behavior.caching = true;
                    config.behavior.cache = sherlock_flags.cache.clone();
                }
                Ok((config, vec![]))
            },
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let mut non_breaking = vec![SherlockError {
                        error: SherlockErrorType::FileExistError(sherlock_flags.config.to_string()),
                        traceback: Default::default(),
                    }];

                    // Unpack non-breaking errors and default config
                    let (mut config, n) = Config::default();
                    non_breaking.extend(n);

                    if sherlock_flags.caching {
                        config.behavior.caching = true;
                        config.behavior.cache = sherlock_flags.cache.clone();
                    }
                    Ok((config, non_breaking))
                }
                _ => Err(SherlockError {
                    error: SherlockErrorType::FileReadError(sherlock_flags.config.to_string()),
                    traceback: e.to_string(),
                })?,
            },
        }
    }
}
