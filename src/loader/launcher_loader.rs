use std::fs;
use std::collections::HashMap;
use std::path::Path;

use crate::launcher::{app_launcher, bulk_text_launcher, calc_launcher, system_cmd_launcher, web_launcher, Launcher, LauncherCommons};
use app_launcher::App;
use web_launcher::Web;
use calc_launcher::Calc;
use system_cmd_launcher::SystemCommand;
use bulk_text_launcher::BulkText;

use super::{util::{self, SherlockError}, Loader};
use util::{CommandConfig, SherlockFlags, AppData, Config};


impl Loader {
    pub fn load_launchers(sherlock_flags: &SherlockFlags, app_config:&Config)->Result<(Vec<Launcher>, Vec<SherlockError>), SherlockError>{
        // Read fallback data here:
        let mut non_breaking: Vec<SherlockError> = Vec::new();

        let (config, n) = parse_launcher_configs(sherlock_flags)?;
        non_breaking.extend(n);

        // Parse the launchers 
        let mut launchers: Vec<Launcher> = config.iter().filter_map(|cmd|{
            let common = LauncherCommons {
                name: cmd.name.to_string(),
                alias: cmd.alias.clone(),
                method: cmd.r#type.clone(),
                priority: cmd.priority,
                r#async: cmd.r#async,
                home: cmd.home,
            };

            match cmd.r#type.as_str(){
                "app_launcher" => {
                    let apps = Loader::load_applications(sherlock_flags, app_config).map_err(|e| non_breaking.push(e)).ok()?;
                    Some(Launcher::App { common, specific: App { apps } })
                }
                "web_launcher" => Some(Launcher::Web { common, specific: Web {
                    icon: cmd.args["icon"].as_str().unwrap_or_default().to_string(),
                    engine: cmd.args["search_engine"].as_str().unwrap_or_default().to_string(),
                }}),
                "calculation" => Some(Launcher::Calc{common, specific: Calc {}}),
                "command" => {
                    let commands: HashMap<String, AppData> = serde_json::from_value(cmd.args["commands"].clone()).unwrap_or_default();                
                    Some(Launcher::SystemCommand {common, specific: SystemCommand { commands }})
                },
                "bulk_text" => {
                    Some(Launcher::BulkText{common, specific: BulkText{
                        icon: cmd.args["icon"].as_str().unwrap_or_default().to_string(),
                        exec: cmd.args["exec"].as_str().unwrap_or_default().to_string(),
                        args: cmd.args["exec-args"].as_str().unwrap_or_default().to_string(),
                    }})
                }
                _ => None
            }
        }).collect();
        launchers.sort_by_key(|s| s.priority());
        Ok((launchers, non_breaking))
    }

}


fn parse_launcher_configs(sherlock_flags: &SherlockFlags)->Result<(Vec<CommandConfig>, Vec<SherlockError>), SherlockError>{
    let mut non_breaking: Vec<SherlockError> = Vec::new();

    fn parse_json(json_str: String) -> Result<Vec<CommandConfig>, SherlockError>{
        if json_str.is_empty() {
            return Ok(Vec::new())
        };

        let config:Vec<CommandConfig> = serde_json::from_str(&json_str.as_str())
            .map_err(|e| SherlockError {
                name: format!("File Parse Error"),
                message: format!("Failed to parse fallback file as valid json."),
                traceback: e.to_string(),
            })?;
        Ok(config)
    }

    // Non breaking (warning) errors as return
    fn load_user_fallback(sherlock_flags: &SherlockFlags)->Result<Vec<CommandConfig>, SherlockError>{
        if Path::new(&sherlock_flags.fallback).exists(){
            let json_str = fs::read_to_string(&sherlock_flags.fallback)
                .map_err(|e| SherlockError {
                    name: format!("File Read Error"),
                    message: format!("Failed to load provided fallback file: {}", sherlock_flags.fallback),
                    traceback: e.to_string(),
                })?;
            let config = parse_json(json_str)?;
            Ok(config)
        } else {
            Err(SherlockError{
                name: "Config not Provided".to_string(),
                message: format!("No launchers were provided. Continuing with default launchers."),
                traceback: format!("Try adding a 'fallback.json' file into '~/.config/sherlock/'. Or specify a custom one using the --falback flag.")
            })
        }
    }

    fn load_default_fallback()->Result<Vec<CommandConfig>, SherlockError>{
        let data = gio::resources_lookup_data("/dev/skxxtz/sherlock/fallback.json", gio::ResourceLookupFlags::NONE)
            .map_err(|e| SherlockError {
                name: format!("Resource Lookup Error"),
                message: format!("Failed to load 'fallback.json' from resource."),
                traceback: e.to_string(),
            })?;
        let string_data = std::str::from_utf8(&data)
            .map_err(|e| SherlockError {
                name: format!("File Parsing Error"),
                message: format!("Failed to parse 'fallback.json' as a valid UTF-8 string."),
                traceback: e.to_string(),
            })?.to_string();
        let config = parse_json(string_data)?;
        Ok(config)
    }


    let config = match load_user_fallback(sherlock_flags).map_err(|e| non_breaking.push(e)).ok(){
        Some(v) => v,
        None => load_default_fallback()?
    };

    return Ok((config, non_breaking))
}

