use serde::de::IntoDeserializer;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

use std::env;
use std::fs::{self, File};
use std::path::PathBuf;

use crate::actions::util::read_from_clipboard;
use crate::launcher::audio_launcher::AudioLauncherFunctions;
use crate::launcher::calc_launcher::Calculator;
use crate::launcher::category_launcher::CategoryLauncher;
use crate::launcher::event_launcher::EventLauncher;
use crate::launcher::process_launcher::ProcessLauncher;
use crate::launcher::weather_launcher::WeatherLauncher;
use crate::launcher::{
    app_launcher, bulk_text_launcher, clipboard_launcher, system_cmd_launcher, web_launcher,
    Launcher, LauncherType,
};
use crate::utils::errors::SherlockError;
use crate::utils::errors::SherlockErrorType;

use app_launcher::App;
use bulk_text_launcher::BulkText;
use clipboard_launcher::ClipboardLauncher;
use simd_json;
use simd_json::prelude::ArrayTrait;
use system_cmd_launcher::SystemCommand;
use web_launcher::Web;

use super::application_loader::parse_priority;
use super::util::deserialize_named_appdata;
use super::util::AppData;
use super::util::RawLauncher;
use super::Loader;
use crate::CONFIG;

impl Loader {
    #[sherlock_macro::timing("Loading launchers")]
    pub fn load_launchers() -> Result<(Vec<Launcher>, Vec<SherlockError>), SherlockError> {
        let config = CONFIG.get().ok_or_else(|| SherlockError {
            error: SherlockErrorType::ConfigError(None),
            traceback: String::new(),
        })?;

        // Read fallback data here:
        let (launcher_config, n) = parse_launcher_configs(&config.files.fallback)?;

        // Read cached counter file
        let counter_reader = CounterReader::new()?;
        let counts = counter_reader.read()?;
        let max_decimals = counts
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(_, v)| v.to_string().len())
            .unwrap_or(0) as i32;

        // Parse the launchers
        let deserialized_launchers: Vec<Result<Launcher, SherlockError>> = launcher_config
            .into_iter()
            .map(|cmd| {
                let counts_clone = counts.clone();
                let launcher_type: LauncherType = match cmd.r#type.as_str() {
                    "categories" => {
                        let prio = cmd.priority;
                        let category_value = &cmd.args["categories"];
                        let mut categories: HashSet<AppData> =
                            deserialize_named_appdata(category_value.clone().into_deserializer())
                                .unwrap_or_default();
                        categories = categories
                            .into_iter()
                            .map(|c| {
                                let count = counts_clone.get(&c.exec).copied().unwrap_or(0.0);
                                c.with_priority(parse_priority(prio, count, max_decimals))
                            })
                            .collect();
                        LauncherType::CategoryLauncher(CategoryLauncher { categories })
                    }
                    "app_launcher" => {
                        let mut apps: HashSet<AppData> = HashSet::new();
                        if let Some(c) = CONFIG.get() {
                            apps = match c.behavior.caching {
                                true => Loader::load_applications(
                                    cmd.priority as f32,
                                    counts_clone,
                                    max_decimals,
                                )?,
                                false => Loader::load_applications_from_disk(
                                    None,
                                    cmd.priority as f32,
                                    counts_clone,
                                    max_decimals,
                                )?,
                            };
                        }

                        LauncherType::App(App { apps })
                    }
                    "web_launcher" => LauncherType::Web(Web {
                        display_name: cmd.display_name.clone().unwrap_or("".to_string()),
                        icon: cmd.args["icon"].as_str().unwrap_or_default().to_string(),
                        engine: cmd.args["search_engine"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                    }),
                    "calculation" => {
                        let capabilities: Option<HashSet<String>> =
                            match cmd.args.get("capabilities") {
                                Some(Value::Array(arr)) => {
                                    let strings: HashSet<String> = arr
                                        .iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect();
                                    Some(strings)
                                }
                                _ => None,
                            };
                        LauncherType::Calc(Calculator { capabilities })
                    }
                    "command" => {
                        let prio = cmd.priority;
                        let commands_value = &cmd.args["commands"];
                        let mut commands: HashSet<AppData> =
                            deserialize_named_appdata(commands_value.clone().into_deserializer())
                                .unwrap_or_default();
                        commands = commands
                            .into_iter()
                            .map(|c| {
                                let count = counts_clone.get(&c.exec).copied().unwrap_or(0.0);
                                c.with_priority(parse_priority(prio, count, max_decimals))
                            })
                            .collect();

                        LauncherType::SystemCommand(SystemCommand { commands })
                    }
                    "bulk_text" => LauncherType::BulkText(BulkText {
                        icon: cmd.args["icon"].as_str().unwrap_or_default().to_string(),
                        exec: cmd.args["exec"].as_str().unwrap_or_default().to_string(),
                        args: cmd.args["exec-args"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                    }),
                    "clipboard-execution" => {
                        let clipboard_content: String = read_from_clipboard()?;
                        let capabilities: Option<HashSet<String>> =
                            match cmd.args.get("capabilities") {
                                Some(Value::Array(arr)) => {
                                    let strings: HashSet<String> = arr
                                        .iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect();
                                    Some(strings)
                                }
                                _ => None,
                            };
                        if clipboard_content.is_empty() {
                            LauncherType::Empty
                        } else {
                            LauncherType::Clipboard((
                                ClipboardLauncher {
                                    clipboard_content,
                                    capabilities: capabilities.clone(),
                                },
                                Calculator { capabilities },
                            ))
                        }
                    }
                    "teams_event" => {
                        let icon = cmd.args["icon"].as_str().unwrap_or("teams").to_string();
                        let date = cmd.args["event_date"].as_str().unwrap_or("now");
                        let event_start = cmd.args["event_start"].as_str().unwrap_or("-5 minutes");
                        let event_end = cmd.args["event_end"].as_str().unwrap_or("+15 minutes");

                        let event = EventLauncher::get_event(date, event_start, event_end);

                        LauncherType::EventLauncher(EventLauncher { event, icon })
                    }
                    "audio_sink" => AudioLauncherFunctions::new()
                        .and_then(|launcher| {
                            launcher.get_current_player().and_then(|player| {
                                launcher.get_metadata(&player).and_then(|launcher| {
                                    Some(LauncherType::MusicPlayerLauncher(launcher))
                                })
                            })
                        })
                        .unwrap_or(LauncherType::Empty),
                    "process" => {
                        let icon = cmd.args["icon"].as_str().unwrap_or("sherlock-process");
                        let launcher = ProcessLauncher::new(icon);
                        if let Some(launcher) = launcher {
                            LauncherType::ProcessLauncher(launcher)
                        } else {
                            LauncherType::Empty
                        }
                    }
                    "weather" => {
                        if let Some(location) = cmd.args["location"].as_str() {
                            let update_interval =
                                cmd.args["update_interval"].as_u64().unwrap_or(60);
                            LauncherType::WeatherLauncher(WeatherLauncher {
                                location: location.to_string(),
                                update_interval,
                            })
                        } else {
                            LauncherType::Empty
                        }
                    }
                    "debug" => {
                        let prio = cmd.priority;
                        let commands_value = &cmd.args["commands"];
                        let mut commands: HashSet<AppData> =
                            deserialize_named_appdata(commands_value.clone().into_deserializer())
                                .unwrap_or_default();
                        commands = commands
                            .into_iter()
                            .map(|c| {
                                let count = counts_clone.get(&c.exec).copied().unwrap_or(0.0);
                                c.with_priority(parse_priority(prio, count, max_decimals))
                            })
                            .collect();

                        LauncherType::SystemCommand(SystemCommand { commands })
                    }
                    _ => LauncherType::Empty,
                };
                let method: String = if let Some(value) = &cmd.on_return {
                    value.to_string()
                } else {
                    cmd.r#type.clone()
                };
                Ok(Launcher {
                    name: cmd.name,
                    alias: cmd.alias,
                    tag_start: cmd.tag_start,
                    tag_end: cmd.tag_end,
                    method,
                    next_content: cmd.next_content,
                    priority: cmd.priority as u32,
                    r#async: cmd.r#async,
                    home: cmd.home,
                    only_home: cmd.only_home,
                    launcher_type,
                    shortcut: cmd.shortcut,
                    spawn_focus: cmd.spawn_focus,
                })
            })
            .collect();

        // Get errors and launchers
        let (oks, errs): (Vec<_>, Vec<_>) =
            deserialized_launchers.into_iter().partition(Result::is_ok);
        let launchers: Vec<Launcher> = oks.into_iter().filter_map(Result::ok).collect();
        let mut non_breaking: Vec<SherlockError> =
            errs.into_iter().filter_map(Result::err).collect();
        if counts.is_empty() {
            let counts: HashMap<String, f32> = launchers
                .iter()
                .filter_map(|launcher| launcher.get_execs())
                .flat_map(|exec_set| exec_set.into_iter().map(|exec| (exec, 0.0)))
                .collect();
            if let Err(e) = counter_reader.write(counts) {
                non_breaking.push(e)
            };
        }
        non_breaking.extend(n);
        Ok((launchers, non_breaking))
    }
}

pub struct CounterReader {
    path: PathBuf,
}
impl CounterReader {
    pub fn new() -> Result<Self, SherlockError> {
        let home = env::var("HOME").map_err(|e| SherlockError {
            error: SherlockErrorType::EnvVarNotFoundError("HOME".to_string()),
            traceback: e.to_string(),
        })?;
        let home_dir = PathBuf::from(home);
        let path = home_dir.join(".sherlock/counts.json");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| SherlockError {
                error: SherlockErrorType::DirCreateError(".sherlock".to_string()),
                traceback: e.to_string(),
            })?;
        }
        Ok(CounterReader { path })
    }
    pub fn write(&self, counts: HashMap<String, f32>) -> Result<(), SherlockError> {
        let tmp_path = self.path.with_extension(".tmp");
        if let Ok(f) = File::create(&tmp_path) {
            if let Ok(_) = simd_json::to_writer(f, &counts) {
                let _ = fs::rename(&tmp_path, &self.path);
            } else {
                let _ = fs::remove_file(&tmp_path);
            }
        }
        Ok(())
    }
    pub fn read(&self) -> Result<HashMap<String, f32>, SherlockError> {
        let file = if self.path.exists() {
            File::open(&self.path)
        } else {
            File::create(&self.path)
        }
        .map_err(|e| SherlockError {
            error: SherlockErrorType::FileExistError(self.path.clone()),
            traceback: e.to_string(),
        })?;
        let counts = match simd_json::from_reader(file).ok() {
            Some(c) => c,
            _ => HashMap::new(),
        };

        Ok(counts)
    }
    pub fn increment(&self, key: &str) -> Result<(), SherlockError> {
        let mut content = self.read()?;
        *content.entry(key.to_string()).or_insert(0.0) += 1.0;
        self.write(content)?;
        Ok(())
    }
}

fn parse_launcher_configs(
    fallback_path: &PathBuf,
) -> Result<(Vec<RawLauncher>, Vec<SherlockError>), SherlockError> {
    // Reads all the configurations of launchers. Either from fallback.json or from default
    // file.

    let mut non_breaking: Vec<SherlockError> = Vec::new();

    fn load_user_fallback(fallback_path: &PathBuf) -> Result<Vec<RawLauncher>, SherlockError> {
        // Tries to load the user-specified launchers. If it failes, it returns a non breaking
        // error.
        match File::open(&fallback_path) {
            Ok(f) => simd_json::from_reader(f).map_err(|e| SherlockError {
                error: SherlockErrorType::FileParseError(fallback_path.clone()),
                traceback: e.to_string(),
            }),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(SherlockError {
                error: SherlockErrorType::FileExistError(fallback_path.clone()),
                traceback: format!(
                    "The file \"{}\" does not exist in the specified location.",
                    fallback_path.to_string_lossy()
                ),
            }),
            Err(e) => Err(SherlockError {
                error: SherlockErrorType::FileReadError(fallback_path.clone()),
                traceback: e.to_string(),
            }),
        }
    }

    fn load_default_fallback() -> Result<Vec<RawLauncher>, SherlockError> {
        // Loads default fallback.json file and loads the launcher configurations within.
        let data = gio::resources_lookup_data(
            "/dev/skxxtz/sherlock/fallback.json",
            gio::ResourceLookupFlags::NONE,
        )
        .map_err(|e| SherlockError {
            error: SherlockErrorType::ResourceLookupError("fallback.json".to_string()),
            traceback: e.to_string(),
        })?;
        let string_data = std::str::from_utf8(&data)
            .map_err(|e| SherlockError {
                error: SherlockErrorType::FileParseError(PathBuf::from("fallback.json")),
                traceback: e.to_string(),
            })?
            .to_string();
        serde_json::from_str(&string_data).map_err(|e| SherlockError {
            error: SherlockErrorType::FileParseError(PathBuf::from("fallback.json")),
            traceback: e.to_string(),
        })
    }

    let config = match load_user_fallback(fallback_path)
        .map_err(|e| non_breaking.push(e))
        .ok()
    {
        Some(v) => v,
        None => load_default_fallback()?,
    };

    return Ok((config, non_breaking));
}
