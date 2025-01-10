use std::collections::HashMap;

use crate::launcher::{Launcher, app_launcher, web_launcher, calc_launcher, system_cmd_launcher, get_api_launcher};
use app_launcher::{App, AppData};
use web_launcher::Web;
use calc_launcher::Calc;
use system_cmd_launcher::SystemCommand;
use get_api_launcher::ApiGet;
use std::path::Path;
use std::{env, fs};

use super::{Loader, util::CommandConfig};


impl Loader {
    pub fn load_launchers()->Vec<Launcher>{
        let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home/user"));
        let user_config_path = format!("{}/.config/sherlock/fallback.json", home_dir);

        // Check if the user has a custom config file
        let json_str = if Path::new(&user_config_path).exists() {
            match fs::read_to_string(&user_config_path){
                Ok(value) => value,
                _ => String::new()
            }
        } else {
            let data = gio::resources_lookup_data("/com/skxxtz/sherlock/fallback.json", gio::ResourceLookupFlags::NONE)
                .expect("Failed to load fallback.json from resources");
            match std::str::from_utf8(&data) {
                Ok(value) => {
                    value.to_string()
                },
                _ => {
                    String::new()
                }
            }
        }; 
        let config: Vec<CommandConfig> = if !json_str.is_empty() {

            serde_json::from_str(&json_str.as_str()).expect("Error parsing fallbacks")
        } else {
            Default::default()
        };

        let mut uuid_counter: u32 = 0;

        let mut launchers: Vec<Launcher> = config.iter().filter_map(|cmd|{
            uuid_counter += 1;

            match cmd.r#type.as_str(){
                "launch_app" => Some(Launcher::App(App {
                    method: "app".to_string(),
                    uuid: format!("app_{}", uuid_counter),
                    name: cmd.name.clone(),
                    alias: cmd.alias.clone(), 
                    priority: cmd.priority,
                    r#async: cmd.r#async,
                    apps: Loader::load_applications(),
                })),
                "web_search" => Some(Launcher::Web(Web {
                    method: "web".to_string(),
                    uuid: format!("web_{}", uuid_counter),
                    name: cmd.name.clone(), 
                    alias: cmd.alias.clone(),
                    icon: cmd.args["icon"].as_str().unwrap_or_default().to_string(),
                    engine: cmd.args["search_engine"].as_str().unwrap_or_default().to_string(),
                    priority: cmd.priority,
                    r#async: cmd.r#async,
                })),
                "calculation" => Some(Launcher::Calc(Calc {
                    method: "calc".to_string(),
                    uuid: format!("calc_{}", uuid_counter),
                    alias: cmd.alias.clone(), 
                    name: cmd.name.clone(), 
                    r#async: cmd.r#async,
                    priority: cmd.priority,
                })),
                "command" => {
                    let commands: HashMap<String, AppData> = serde_json::from_value(cmd.args["commands"].clone()).unwrap_or_default();                
                    Some(Launcher::SystemCommand(SystemCommand {
                        method: "command".to_string(),
                        uuid: format!("command_{}", uuid_counter),
                        name: cmd.name.clone(),
                        alias: cmd.alias.clone(), 
                        priority: cmd.priority,
                        r#async: cmd.r#async,
                        commands,
                    }))

                },
                "get_api" => {
                    Some(Launcher::ApiGet(ApiGet{
                        method: "get_api".to_string(),
                        uuid: format!("get_api_{}", uuid_counter),
                        alias: cmd.alias.clone(),
                        name: cmd.name.clone(),
                        priority: cmd.priority,
                        r#async: cmd.r#async,

                        title_key: cmd.args["title-key"].as_str().unwrap_or_default().to_string(),
                        body_key: cmd.args["body-key"].as_str().unwrap_or_default().to_string(),
                        icon: cmd.args["icon"].as_str().unwrap_or_default().to_string(),
                        url: cmd.args["url"].as_str().unwrap_or_default().to_string(),
                    }))
                }
                _ => None
            }
        }).collect();
        launchers.sort_by_key(|s| s.priority());
        launchers
    }

}
