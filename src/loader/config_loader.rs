use std::fs;
use std::path::Path;

use super::util::{Config, SherlockError, SherlockFlags, SherlockErrorType};
use super::Loader;

impl Loader {
    pub fn load_config(
        sherlock_flags: &SherlockFlags,
    ) -> Result<(Config, Vec<SherlockError>), SherlockError> {
        let mut non_breaking: Vec<SherlockError> = Vec::new();
        let user_config_path = sherlock_flags.config.clone();

        let user_config: Config = if Path::new(&user_config_path).exists() {
            let config_str = fs::read_to_string(&user_config_path).map_err(|e| SherlockError {
                error: SherlockErrorType::FileReadError(user_config_path.clone()),
                traceback: e.to_string(),
            })?;

            toml::de::from_str(&config_str).map_err(move |e| SherlockError {
                error: SherlockErrorType::FileParseError(user_config_path),
                traceback: e.to_string(),
            })?
        } else {
            non_breaking.push(SherlockError {
                error: SherlockErrorType::FileExistError(user_config_path),
                traceback: Default::default(),
            });

            // Unpack non-breaking errors and default config
            let (config, n) = Config::default();
            non_breaking.extend(n);
            config
        };
        Ok((user_config, non_breaking))
    }
}
