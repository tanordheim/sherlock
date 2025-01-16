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
    pub fn load_launchers(sherlock_flags: &SherlockFlags, app_config:&Config)->Result<Vec<Launcher>, SherlockError>{
        // Read fallback data here:
        let json_str = if Path::new(&sherlock_flags.fallback).exists() {
            fs::read_to_string(&sherlock_flags.fallback)
                .map_err(|e| SherlockError {
                    name: format!("File Read Error"),
                    message: format!("Failed to load provided fallback file: {}", sherlock_flags.fallback),
                    traceback: e.to_string(),
                })?
        } else {
            let data = gio::resources_lookup_data("/dev/skxxtz/sherlock/fallback.json", gio::ResourceLookupFlags::NONE)
                .map_err(|e| SherlockError {
                    name: format!("Resource Lookup Error"),
                    message: format!("Failed to load 'fallback.json' from resource."),
                    traceback: e.to_string(),
                })?;
            std::str::from_utf8(&data)
                .map_err(|e| SherlockError {
                    name: format!("File Parsing Error"),
                    message: format!("Failed to parse 'fallback.json' as a valid UTF-8 string."),
                    traceback: e.to_string(),
                })?.to_string()
        }; 

        if json_str.is_empty() {
            return Ok(Vec::new())
        };


        let config:Vec<CommandConfig> = serde_json::from_str(&json_str.as_str())
            .map_err(|e| SherlockError {
                name: format!("File Parse Error"),
                message: format!("Failed to parse fallback file as valid json."),
                traceback: e.to_string(),
            })?;

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
                "app_launcher" => Some(Launcher::App { common, specific: App {apps: Loader::load_applications(sherlock_flags, app_config)}}),
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
        Ok(launchers)
    }

}
