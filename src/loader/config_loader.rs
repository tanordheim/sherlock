use std::fs;
use std::path::PathBuf;

use super::util::{home_dir, SherlockConfig, SherlockError, SherlockErrorType, SherlockFlags};
use super::Loader;
use crate::FLAGS;

impl Loader {
    pub fn load_config() -> Result<(SherlockConfig, Vec<SherlockError>), SherlockError> {
        let sherlock_flags = FLAGS.get().ok_or_else(|| SherlockError {
            error: SherlockErrorType::ConfigError(None),
            traceback: String::new(),
        })?;
        let home = home_dir()?;
        let path = home.join(".config/sherlock/config.toml");

        match fs::read_to_string(&path) {
            Ok(config_str) => {
                let mut config: SherlockConfig = match toml::de::from_str(&config_str) {
                    Ok(config) => config,
                    Err(e) => {
                        return Err(SherlockError {
                            error: SherlockErrorType::FileParseError(path),
                            traceback: e.to_string(),
                        })
                    }
                };

                config = Loader::apply_flags(sherlock_flags, config)?;
                Ok((config, vec![]))
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let mut non_breaking = vec![SherlockError {
                        error: SherlockErrorType::FileExistError(path),
                        traceback: Default::default(),
                    }];

                    // Unpack non-breaking errors and default config
                    let (mut config, n) = SherlockConfig::default();
                    non_breaking.extend(n);

                    config = Loader::apply_flags(sherlock_flags, config)?;
                    Ok((config, non_breaking))
                }
                _ => Err(SherlockError {
                    error: SherlockErrorType::FileReadError(path),
                    traceback: e.to_string(),
                })?,
            },
        }
    }
    fn apply_flags(
        sherlock_flags: &SherlockFlags,
        mut config: SherlockConfig,
    ) -> Result<SherlockConfig, SherlockError> {
        // Make paths that contain the ~ dir use the correct path
        let home = home_dir()?;

        fn expand_home(path: &str, home: &PathBuf) -> PathBuf {
            if let Some(stripped) = path.strip_prefix("~") {
                home.join(stripped.strip_prefix("/").unwrap_or(stripped))
            } else {
                PathBuf::from(path)
            }
        }

        // Override config files from flags
        config.files.config = expand_home(
            &sherlock_flags
                .config
                .as_deref()
                .unwrap_or("~/.config/sherlock/config.toml"),
            &home,
        );
        config.files.fallback = expand_home(
            &sherlock_flags
                .fallback
                .as_deref()
                .unwrap_or("~/.config/sherlock/fallback.json"),
            &home,
        );
        config.files.css = expand_home(
            &sherlock_flags
                .style
                .as_deref()
                .unwrap_or("~/.config/sherlock/main.css"),
            &home,
        );
        config.files.alias = expand_home(
            &sherlock_flags
                .alias
                .as_deref()
                .unwrap_or("~/.config/sherlock/sherlock_alias.json"),
            &home,
        );
        config.files.ignore = expand_home(
            &sherlock_flags
                .ignore
                .as_deref()
                .unwrap_or("~/.config/sherlock/sherlockignore"),
            &home,
        );
        config.behavior.cache = expand_home(
            &sherlock_flags
                .cache
                .as_deref()
                .unwrap_or("~/.cache/sherlock_desktop_cache.json"),
            &home,
        );

        if sherlock_flags.daemonize {
            config.behavior.daemonize = true;
        }
        Ok(config)
    }
}
