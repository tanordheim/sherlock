use gtk4::ListBoxRow;

pub mod app_launcher;
pub mod web_launcher;
pub mod calc_launcher;
pub mod get_api_launcher;
pub mod system_cmd_launcher;

use crate::ui::tiles::Tile;

use app_launcher::App;
use web_launcher::Web;
use calc_launcher::Calc;
use get_api_launcher::ApiGet;
use system_cmd_launcher::SystemCommand;

#[derive(Clone, Debug)]
pub enum Launcher{
    App(App),
    Web(Web),
    Calc(Calc),
    ApiGet(ApiGet),
    SystemCommand(SystemCommand),
}
impl Launcher{
    fn get_patch(&self, index:i32, keyword: &String)->(i32, Vec<ListBoxRow>){
        match self {
            Launcher::App(app) => Tile::app_tile(index, app.apps.clone(), &app.name, &app.method, keyword),
            Launcher::Web(web) => Tile::web_tile(&web.name, &web.method, &web.engine, index, keyword),
            Launcher::Calc(calc) => Tile::calc_tile(index, keyword, &calc.method),
            Launcher::ApiGet(api) => Tile::bulk_text_tile(&api.name, &api.method, &api.icon, &api.url, &api.key, index, keyword),
            Launcher::SystemCommand(cmd) => Tile::app_tile(index, cmd.commands.clone(), &cmd.name, &cmd.method, keyword),
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

pub fn construct_tiles(keyword: &String, launchers: &[Launcher], mode: &String)->Vec<ListBoxRow>{
    let mut widgets = Vec::with_capacity(launchers.len());
    let sel_mode = mode.trim();
    let mut index:i32 = 0;
    for launcher in launchers.iter() {
        let alias = launcher.alias();
        if launcher.priority() == 0 && alias != sel_mode {
            continue;
        } 
        
        if alias == sel_mode || sel_mode == "all" {
            let (returned_index, result) = launcher.get_patch(index, keyword);
            index = returned_index;
            widgets.extend(result); 
            
        }
    }
    widgets
}

