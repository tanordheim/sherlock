use std::collections::HashMap;

use crate::launcher::{Launcher, app_launcher, web_launcher, calc_launcher, system_cmd_launcher, get_api_launcher};
use app_launcher::{App, AppData};
use web_launcher::Web;
use calc_launcher::Calc;
use system_cmd_launcher::SystemCommand;
use get_api_launcher::ApiGet;

use super::{Loader, util::CommandConfig};


impl Loader {
    pub fn load_launchers()->Vec<Launcher>{
        let data = gio::resources_lookup_data("/com/skxxtz/sherlock/fallback.json", gio::ResourceLookupFlags::NONE)
            .expect("Failed to load fallback.json from resources");
        let json_str = std::str::from_utf8(&data)
            .expect("Failed to parse string from fallback.json resource");
        let config:Vec<CommandConfig> = serde_json::from_str(&json_str).expect("Error parsing fallbacks");

        let mut launchers: Vec<Launcher> = config.iter().map(|cmd|{
            match cmd.r#type.as_str(){
                "launch_app" => Launcher::App(App {
                    method: "app".to_string(),
                    name: cmd.name.clone(),
                    alias: cmd.alias.clone(), 
                    priority: cmd.priority,
                    apps: Loader::load_applications(),
                }),
                "web_search" => Launcher::Web(Web {
                    method: "web".to_string(),
                    name: cmd.name.clone(), 
                    alias: cmd.alias.clone(), 
                    engine: cmd.args["search_engine"].as_str().unwrap_or_default().to_string(),
                    priority: cmd.priority,
                }),
                "calculation" => Launcher::Calc(Calc {
                    method: "calc".to_string(),
                    alias: cmd.alias.clone(), 
                    name: cmd.name.clone(), 
                    priority: cmd.priority,
                }),
                "command" => {
                    let commands: HashMap<String, AppData> = serde_json::from_value(cmd.args["commands"].clone()).unwrap_or_default();                
                    Launcher::SystemCommand(SystemCommand {
                        method: "command".to_string(),
                        name: cmd.name.clone(),
                        alias: cmd.alias.clone(), 
                        priority: cmd.priority,
                        commands,
                    })

                },
                "get_api" => {
                    Launcher::ApiGet(ApiGet{
                        method: "get_api".to_string(),
                        alias: cmd.alias.clone(),
                        name: cmd.name.clone(),
                        priority: cmd.priority,
                        
                        key: cmd.args["key"].as_str().unwrap_or_default().to_string(),
                        icon: cmd.args["icon"].as_str().unwrap_or_default().to_string(),
                        url: cmd.args["url"].as_str().unwrap_or_default().to_string(),
                    })
                }
                _ => {
                    eprint!("Unknown command type: {}", cmd.r#type);
                    Launcher::App(App {
                        method: String::new(),
                        name: String::new(),
                        alias: None,
                        priority: 0,
                        apps: Default::default(),
                    })
                }
            }
        }).collect();
        launchers.sort_by_key(|s| s.priority());
        launchers
    }

}
