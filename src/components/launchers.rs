use gtk4::prelude::WidgetExt;
use gtk4::{prelude::BoxExt, Box, Builder, Image, Label, ListBoxRow};
use serde::Deserialize;
use std::{collections::HashMap, fs};
use std::{env, path::Path};
use meval::eval_str;


use crate::helpers::{get_applications, AppData};


pub fn launcher_loop(keyword: &String, launchers: &[Launcher], mode: &String) -> Vec<ListBoxRow> {
    let mut widgets = Vec::with_capacity(launchers.len());
    let sel_mode = mode.trim();
    let mut index:i32 = 0;
    for launcher in launchers.iter() {
        let alias = launcher.alias();
        
        if alias == sel_mode || sel_mode == "all" {
            let (returned_index, result) = launcher.get_patch(index, keyword);
            index = returned_index;
            widgets.extend(result); 
            
        }
    }
    widgets
}
pub fn get_launchers()->Vec<Launcher>{
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
                apps: get_applications(),
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




// Deserializer
#[derive(Deserialize, Debug)]
struct CommandConfig {
    name: String,
    alias: Option<String>,
    r#type: String,
    args: serde_json::Value,
    priority: u32,
}

#[derive(Clone, Debug)]
pub enum Launcher{
    App(App),
    Web(Web),
    Calc(Calc),
    ApiGet(ApiGet),
    SystemCommand(SystemCommand),
}
#[derive(Clone, Debug)]
struct App{
    pub alias: Option<String>,
    method: String ,
    name: String,
    priority: u32,
    apps: HashMap<String, AppData>,
}
#[derive(Clone, Debug)]
struct Web{
    pub alias: Option<String>,
    method: String ,
    name: String,
    engine: String,
    priority: u32,
}
#[derive(Clone, Debug)]
struct Calc{
    pub alias: Option<String>,
    method: String ,
    name: String,
    priority: u32,
}
#[derive(Clone, Debug)]
struct ApiGet{
    pub alias: Option<String>,
    method: String,
    name: String,
    path: String,
    key: String,
    priority: u32,
}
#[derive(Clone, Debug)]
struct SystemCommand{
    pub alias: Option<String>,
    method: String,
    name: String,
    commands: HashMap<String, AppData>,
    priority: u32,
}

fn insert_attrs(attr_holder:&Box, attrs:Vec<String>){
    for item in attrs{
        let label = Label::new(None);
        label.set_text(&item);
        attr_holder.append(&label);
    }
}

impl Web{
    fn web_tile(&self,index:i32, keyword:&String)->(i32, Vec<ListBoxRow>){
        if !keyword.is_empty(){
            let builder = Builder::from_resource("/com/skxxtz/sherlock/ui/tile.ui");
            let holder:ListBoxRow = builder.object("holder").unwrap();
            let icon_obj:Image = builder.object("icon-name").unwrap();
            let title_obj:Label = builder.object("app-name").unwrap();
            let launcher_type:Label = builder.object("launcher-type").unwrap();
            let attr_holder:Box = builder.object("attrs-holder").unwrap();

            if index < 5 {
                let shortcut_holder:Box = builder.object("shortcut-holder").unwrap();
                let shortcut:Label = builder.object("shortcut").unwrap();
                shortcut_holder.set_visible(true);
                shortcut.set_text(format!("ctrl + {}", index + 1).as_str());
            }

            launcher_type.set_text(&self.name);
            icon_obj.set_icon_name(Some("google"));
            title_obj.set_text(keyword);

            let attrs: Vec<String> = vec![
                format!("{} | {}", "method", self.method),
                format!("{} | {}", "engine", self.engine),
                format!("{} | {}", "keyword", keyword),
            ];
            insert_attrs(&attr_holder, attrs);

            return (index + 1, vec![holder])
        }
        (index, vec![])
    }
}
impl ApiGet{
    fn bulk_text_tile(&self, index:i32, keyword:&String)->(i32, Vec<ListBoxRow>){
        if !keyword.is_empty(){
            let builder = Builder::from_resource("/com/skxxtz/sherlock/ui/wiki_tile.ui");
            let holder:ListBoxRow = builder.object("holder").unwrap();
            let icon_obj:Image = builder.object("icon-name").unwrap();
            let title_obj:Label = builder.object("app-name").unwrap();
            let launcher_type:Label = builder.object("launcher-type").unwrap();
            let attr_holder:Box = builder.object("attrs-holder").unwrap();

            launcher_type.set_text(&self.name);
            icon_obj.set_icon_name(Some("system-shutdown"));
            title_obj.set_text(keyword);


            if index < 5 {
                let shortcut_holder:Label = builder.object("shortcut-holder").unwrap();
                let shortcut:Label = builder.object("shortcut").unwrap();
                shortcut_holder.set_visible(true);
                shortcut.set_text(format!("ctrl + {}", index + 1).as_str());
            }

            let attrs: Vec<String> = vec![
                format!("{} | {}", "method", self.method),
                format!("{} | {}", "keyword", keyword),
            ];
            insert_attrs(&attr_holder, attrs);

            return (index + 1, vec![holder])
        }
        (index, vec![])

    }
}




struct Tiles{}
impl Tiles { 
    fn app_tile(index:i32, commands:HashMap<String, AppData>, name:&String, method:&String, keyword:&String)->(i32, Vec<ListBoxRow>){
        let mut results: Vec<ListBoxRow> = Default::default();
        let mut index_ref = index;

        for (key, value) in commands.into_iter() {
            if key.to_lowercase().contains(&keyword.to_lowercase()){
                let builder = Builder::from_resource("/com/skxxtz/sherlock/ui/tile.ui");
                let holder:ListBoxRow = builder.object("holder").unwrap();
                let icon_obj:Image = builder.object("icon-name").unwrap();
                let title_obj:Label = builder.object("app-name").unwrap();
                let attr_holder:Box = builder.object("attrs-holder").unwrap();

                if index_ref < 5 {
                    let shortcut_holder:Box = builder.object("shortcut-holder").unwrap();
                    let shortcut:Label = builder.object("shortcut").unwrap();
                    shortcut_holder.set_visible(true);
                    shortcut.set_text(format!("ctrl + {}", index_ref + 1).as_str());
                }

                let launcher_type:Label = builder.object("launcher-type").unwrap();
                launcher_type.set_text(name);
                icon_obj.set_icon_name(Some(&value.icon));
                title_obj.set_text(&key);


                let attrs: Vec<String> = vec![
                    format!("{} | {}", "method", method),
                    format!("{} | {}", "app_name", &key),
                    format!("{} | {}", "exec", &value.exec),
                ];
                insert_attrs(&attr_holder, attrs);
                index_ref += 1;

                results.push(holder);
            }

        }
        return (index_ref, results)
    }
    fn calc_tile(index:i32, equation:&str, method:&String)->(i32, Vec<ListBoxRow>){
        let mut results: Vec<ListBoxRow> = Default::default();
        match eval_str(equation){
            Ok(result)=> {
                let builder = Builder::from_resource("/com/skxxtz/sherlock/ui/calc_tile.ui");

                let holder:ListBoxRow = builder.object("holder").unwrap();
                let attr_holder:Box = builder.object("attrs-holder").unwrap();

                let equation_holder:Label = builder.object("equation-holder").unwrap();
                let result_holder:Label = builder.object("result-holder").unwrap();

                equation_holder.set_text(&equation);
                result_holder.set_text(format!("= {}", result.to_string()).as_str());

                let attrs: Vec<String> = vec![
                    format!("{} | {}", "method", method),
                    format!("{} | {}", "result", result),
                ];
                insert_attrs(&attr_holder, attrs);
                
                results.push(holder);

            }
            _ => {}
        }

        (index, results)
    }
}





impl Launcher{
    fn get_patch(&self, index:i32, keyword: &String)->(i32, Vec<ListBoxRow>){
        match self {
            Launcher::App(app) => Tiles::app_tile(index, app.apps.clone(), &app.name, &app.method, keyword),
            Launcher::Web(web) => Web::web_tile(web,index, keyword),
            Launcher::Calc(calc) => Tiles::calc_tile(index, keyword, &calc.method),
            Launcher::ApiGet(api) => ApiGet::bulk_text_tile(api, index, keyword),
            Launcher::SystemCommand(cmd) => Tiles::app_tile(index, cmd.commands.clone(), &cmd.name, &cmd.method, keyword),
        }
    }
    pub fn priority(&self)->u32{
        match self {
            Launcher::App(app) => app.priority,
            Launcher::Web(web) => web.priority,
            Launcher::Calc(calc) => calc.priority,
            Launcher::ApiGet(api) => api.priority,
            Launcher::SystemCommand(cmd) => cmd.priority,
        }
    }
    pub fn alias(&self)->String{
        match self {
            Launcher::App(app) => app.alias.clone().unwrap_or_default(),
            Launcher::Web(web) => web.alias.clone().unwrap_or_default(),
            Launcher::Calc(calc) => calc.alias.clone().unwrap_or_default(),
            Launcher::ApiGet(api) => api.alias.clone().unwrap_or_default(),
            Launcher::SystemCommand(cmd) => cmd.alias.clone().unwrap_or_default(),
        }
    }
    pub fn name(&self)->String{
        match self {
            Launcher::App(app) => app.name.clone(),
            Launcher::Web(web) => web.name.clone(),
            Launcher::Calc(calc) => calc.name.clone(),
            Launcher::ApiGet(api) => api.name.clone(),
            Launcher::SystemCommand(cmd) => cmd.name.clone(),
        }
    }
}
