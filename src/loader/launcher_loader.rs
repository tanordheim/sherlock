use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::actions::util::read_from_clipboard;
use crate::launcher::{
    app_launcher, bulk_text_launcher, calc_launcher, clipboard_launcher, system_cmd_launcher,
    web_launcher, Launcher, LauncherType,
};

use app_launcher::App;
use bulk_text_launcher::BulkText;
use calc_launcher::Calc;
use clipboard_launcher::Clp;
use system_cmd_launcher::SystemCommand;
use web_launcher::Web;


use super::{
    util::{self, SherlockError, SherlockErrorType},
    Loader,
};
use util::{AppData, CommandConfig, SherlockFlags};

impl Loader {
    pub fn load_launchers(
        sherlock_flags: &SherlockFlags,
    ) -> Result<(Vec<Launcher>, Vec<SherlockError>), SherlockError> {
        // Read fallback data here:
        let mut non_breaking: Vec<SherlockError> = Vec::new();
        // Read fallback data here:
        let (config, n) = parse_launcher_configs(sherlock_flags)?;
        non_breaking.extend(n);

        // Parse the launchers
        let mut launchers: Vec<Launcher> = config
            .iter()
            .filter_map(|cmd| {
                let launcher_type: LauncherType = match cmd.r#type.as_str() {
                    "app_launcher" => {
                        let apps = Loader::load_applications(sherlock_flags)
                            .map_err(|e| non_breaking.push(e))
                            .ok()?;
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
                    "calculation" => LauncherType::Calc(Calc {}),
                    "command" => {
                        let commands: HashMap<String, AppData> =
                            serde_json::from_value(cmd.args["commands"].clone())
                                .unwrap_or_default();
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
                        let clipboard_content: String = read_from_clipboard()
                            .map_err(|e| non_breaking.push(e))
                            .unwrap_or_default();
                        if clipboard_content.is_empty() {
                            LauncherType::Empty
                        } else {
                            LauncherType::Clipboard(Clp { clipboard_content })
                        }
                    }
                    _ => LauncherType::Empty,
                };
                let method: String = if let Some(value) = &cmd.on_return {
                    value.to_string()
                } else {
                    cmd.r#type.clone()
                };
                Some(Launcher {
                    name: cmd.name.to_string(),
                    alias: cmd.alias.clone(),
                    start_tag: cmd.start_tag,
                    end_tag: cmd.end_tag,
                    method,
                    priority: cmd.priority,
                    r#async: cmd.r#async,
                    home: cmd.home,
                    launcher_type,
                })
            })
            .collect();
        launchers.sort_by_key(|s| s.priority);
        Ok((launchers, non_breaking))
    }
}

fn parse_launcher_configs(
    sherlock_flags: &SherlockFlags,
) -> Result<(Vec<CommandConfig>, Vec<SherlockError>), SherlockError> {
    // Reads all the configurations of launchers. Either from fallback.json or from default
    // file.

    let mut non_breaking: Vec<SherlockError> = Vec::new();

    fn parse_json(json_str: String, file:String) -> Result<Vec<CommandConfig>, SherlockError> {
        if json_str.is_empty() {
            return Ok(Vec::new());
        };

        let config: Vec<CommandConfig> =
            serde_json::from_str(&json_str.as_str()).map_err(|e| SherlockError {
                error: SherlockErrorType::FileParseError(file),
                traceback: e.to_string(),
            })?;
        Ok(config)
    }

    fn load_user_fallback(
        sherlock_flags: &SherlockFlags,
    ) -> Result<Vec<CommandConfig>, SherlockError> {
        // Tries to load the user-specified launchers. If it failes, it returns a non breaking
        // error.
        if Path::new(&sherlock_flags.fallback).exists() {
            let json_str =
                fs::read_to_string(&sherlock_flags.fallback).map_err(|e| SherlockError {
                    error: SherlockErrorType::FileReadError(sherlock_flags.fallback.clone()),
                    traceback: e.to_string(),
                })?;
            let config = parse_json(json_str, sherlock_flags.fallback.clone())?;
            Ok(config)
        } else {
            Err(SherlockError{
                error: SherlockErrorType::FileExistError(sherlock_flags.fallback.clone()),
                traceback: format!("The file \"{}\" does not exist in the specified location.", sherlock_flags.fallback)
            })
        }
    }

    fn load_default_fallback() -> Result<Vec<CommandConfig>, SherlockError> {
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
                error: SherlockErrorType::FileParseError("fallback.json".to_string()),
                traceback: e.to_string(),
            })?
            .to_string();
        let config = parse_json(string_data, "fallback.json".to_string())?;
        Ok(config)
    }

    let config = match load_user_fallback(sherlock_flags)
        .map_err(|e| non_breaking.push(e))
        .ok()
    {
        Some(v) => v,
        None => load_default_fallback()?,
    };

    return Ok((config, non_breaking));
}
