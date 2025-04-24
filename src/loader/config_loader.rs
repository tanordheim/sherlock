use std::fs;
use std::path::PathBuf;

use super::util::{expand_path, home_dir, SherlockConfig, SherlockFlags};
use super::Loader;
use crate::utils::errors::{SherlockError, SherlockErrorType};

impl Loader {
    pub fn load_config(
        sherlock_flags: &SherlockFlags,
    ) -> Result<(SherlockConfig, Vec<SherlockError>), SherlockError> {
        let home = home_dir()?;
        let mut path = match &sherlock_flags.config {
            Some(path) => expand_path(path, &home),
            _ => home.join(".config/sherlock/config.toml"),
        };
        // logic to either use json or toml
        let mut filetype: String = String::new();
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy();
            match ext.as_ref() {
                "json" => {
                    if !path.exists() {
                        path.set_extension("toml");
                        filetype = "toml".to_string();
                    } else {
                        filetype = "json".to_string();
                    }
                }
                "toml" => {
                    if !path.exists() {
                        path.set_extension("json");
                        filetype = "json".to_string();
                    } else {
                        filetype = "toml".to_string();
                    }
                }
                _ => {}
            }
        } else {
            return Err(SherlockError {
                error: SherlockErrorType::FileParseError(path.clone()),
                traceback: format!(
                    "The file \"{}\" is not in a valid format.",
                    &path.to_string_lossy()
                ),
            });
        }

        match fs::read_to_string(&path) {
            Ok(config_str) => {
                let config_res: Result<SherlockConfig, SherlockError> = match filetype.as_str() {
                    "json" => {
                        let mut bytes = config_str.into_bytes();
                        simd_json::from_slice(&mut bytes).map_err(|e| SherlockError {
                            error: SherlockErrorType::FileParseError(path.clone()),
                            traceback: e.to_string(),
                        })
                    }
                    "toml" => toml::de::from_str(&config_str).map_err(|e| SherlockError {
                        error: SherlockErrorType::FileParseError(path.clone()),
                        traceback: e.to_string(),
                    }),
                    _ => {
                        return Err(SherlockError {
                            error: SherlockErrorType::FileParseError(path.clone()),
                            traceback: format!(
                                "The file \"{}\" is not in a valid format.",
                                &path.to_string_lossy()
                            ),
                        })
                    }
                };
                match config_res {
                    Ok(mut config) => {
                        config = Loader::apply_flags(sherlock_flags, config);
                        return Ok((config, vec![]));
                    }
                    Err(e) => {
                        let mut config = SherlockConfig::default();

                        config = Loader::apply_flags(sherlock_flags, config);
                        Ok((config, vec![e]))
                    }
                }
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let error = SherlockError {
                        error: SherlockErrorType::FileExistError(path),
                        traceback: e.to_string(),
                    };

                    let mut config = SherlockConfig::default();

                    config = Loader::apply_flags(sherlock_flags, config);
                    Ok((config, vec![error]))
                }
                _ => Err(SherlockError {
                    error: SherlockErrorType::FileReadError(path),
                    traceback: e.to_string(),
                })?,
            },
        }
    }
    pub fn apply_flags(
        sherlock_flags: &SherlockFlags,
        mut config: SherlockConfig,
    ) -> SherlockConfig {
        // Make paths that contain the ~ dir use the correct path
        let home = match home_dir() {
            Ok(h) => h,
            Err(_) => return config,
        };

        // Override config files from flags
        config.files.config = expand_path(
            &sherlock_flags
                .config
                .as_deref()
                .unwrap_or(&config.files.config),
            &home,
        );
        config.files.fallback = expand_path(
            &sherlock_flags
                .fallback
                .as_deref()
                .unwrap_or(&config.files.fallback),
            &home,
        );
        config.files.css = expand_path(
            &sherlock_flags.style.as_deref().unwrap_or(&config.files.css),
            &home,
        );
        config.files.alias = expand_path(
            &sherlock_flags
                .alias
                .as_deref()
                .unwrap_or(&config.files.alias),
            &home,
        );
        config.files.ignore = expand_path(
            &sherlock_flags
                .ignore
                .as_deref()
                .unwrap_or(&config.files.ignore),
            &home,
        );
        config.behavior.cache = expand_path(
            &sherlock_flags
                .cache
                .as_deref()
                .unwrap_or(&config.behavior.cache),
            &home,
        );
        config.behavior.sub_menu = sherlock_flags.sub_menu.clone();
        config.pipe.method = sherlock_flags.method.clone();
        config.behavior.field = sherlock_flags.field.clone();

        if sherlock_flags.daemonize {
            config.behavior.daemonize = true;
        }
        config
    }

    /// # Arguments
    /// loc: PathBuf
    /// Pathbuf should be a directory **not** a file
    pub fn write_defaults_to_file(loc: PathBuf) -> Result<(), SherlockError> {
        let create_dir = |path: PathBuf| {
            fs::create_dir(&path).map_err(|e| SherlockError {
                error: SherlockErrorType::DirCreateError(format!("{:?}", path)),
                traceback: e.to_string(),
            })
        };

        // create config location
        let home = home_dir()?;
        let path = expand_path(&loc, &home);

        // build default config
        let config = SherlockConfig::default();
        let toml_str = toml::to_string(&config).map_err(|e| SherlockError {
            error: SherlockErrorType::FileWriteError(path.clone()),
            traceback: e.to_string(),
        })?;

        // mkdir -p
        fs::create_dir_all(&path).map_err(|e| SherlockError {
            error: SherlockErrorType::DirCreateError(format!("{:?}", path)),
            traceback: e.to_string(),
        })?;
        // create subdirs
        create_dir(path.join("icons/"))?;
        create_dir(path.join("scripts/"))?;
        create_dir(path.join("themes/"))?;

        // write config.toml file
        let config_path = path.join("config.toml");
        if !config_path.exists() {
            fs::write(&config_path, toml_str).map_err(|e| SherlockError {
                error: SherlockErrorType::FileWriteError(config_path),
                traceback: e.to_string(),
            })?;
        } else {
            println!("Skipping 'config.toml' since file exists already.")
        }

        // write sherlockignore file
        let ignore_path = path.join("sherlockignore");
        if !ignore_path.exists() {
            fs::write(&ignore_path, "").map_err(|e| SherlockError {
                error: SherlockErrorType::FileWriteError(ignore_path),
                traceback: e.to_string(),
            })?;
        } else {
            println!("Skipping 'sherlockignore' since file exists already.")
        }

        // write sherlock_alias file
        let alias_path = path.join("sherlock_alias.json");
        if !alias_path.exists() {
            fs::write(&alias_path, "{}").map_err(|e| SherlockError {
                error: SherlockErrorType::FileWriteError(alias_path),
                traceback: e.to_string(),
            })?;
        } else {
            println!("Skipping 'sherlock_alias.json' since file exists already.")
        }

        // write main.css file
        let css_path = path.join("main.css");
        if !css_path.exists() {
            fs::write(&css_path, "").map_err(|e| SherlockError {
                error: SherlockErrorType::FileWriteError(css_path),
                traceback: e.to_string(),
            })?;
        } else {
            println!("Skipping 'main.css' since file exists already.")
        }

        // load default fallbacks
        let fallback_path = path.join("fallback.json");
        if !fallback_path.exists() {
            // load resources
            Loader::load_resources()?;
            let data = gio::resources_lookup_data(
                "/dev/skxxtz/sherlock/fallback.json",
                gio::ResourceLookupFlags::NONE,
            )
            .map_err(|e| SherlockError {
                error: SherlockErrorType::ResourceLookupError("fallback.json".to_string()),
                traceback: e.to_string(),
            })?;

            let json_str = std::str::from_utf8(&data).map_err(|e| SherlockError {
                error: SherlockErrorType::FileParseError(PathBuf::from("fallback.json")),
                traceback: e.to_string(),
            })?;
            fs::write(&fallback_path, json_str).map_err(|e| SherlockError {
                error: SherlockErrorType::FileWriteError(fallback_path),
                traceback: e.to_string(),
            })?;
        } else {
            println!("Skipping 'fallback.json' since file exists already.")
        }

        std::process::exit(0);
    }
}
