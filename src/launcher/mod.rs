use gtk4::{Label, ListBoxRow, TextView, Box as HVBox};

pub mod app_launcher;
pub mod web_launcher;
pub mod calc_launcher;
pub mod bulk_text_launcher;
pub mod system_cmd_launcher;

use crate::ui::tiles::Tile;

use app_launcher::App;
use web_launcher::Web;
use calc_launcher::Calc;
use bulk_text_launcher::BulkText;
use system_cmd_launcher::SystemCommand;

#[derive(Clone, Debug)]
pub enum Launcher{
    App(App),
    Web(Web),
    Calc(Calc),
    BulkText(BulkText),
    SystemCommand(SystemCommand),
}
impl Launcher{
    pub fn get_loader_widget(&self, keyword: &String)-> Option<(ListBoxRow, Label, TextView, HVBox)>{
        match self {
            Launcher::App(_) => None,
            Launcher::Web(_) => None,
            Launcher::Calc(_) => None,
            Launcher::BulkText(api) => Tile::bulk_text_tile_loader(&api.name, &api.method, &api.icon, keyword),
            Launcher::SystemCommand(_) => None,
        }
        
    }
    fn get_patch(&self, index:i32, keyword: &String)->(i32, Vec<ListBoxRow>){
        match self {
            Launcher::App(app) => Tile::app_tile(index, app.apps.clone(), &app.name, &app.method, keyword),
            Launcher::Web(web) => Tile::web_tile(&web.name, &web.method, &web.icon, &web.engine, index, keyword),
            Launcher::Calc(calc) => Tile::calc_tile(index, keyword, &calc.method),
            Launcher::BulkText(api) => Tile::bulk_text_tile(&api.name, &api.method, &api.icon, index, keyword),
            Launcher::SystemCommand(cmd) => Tile::app_tile(index, cmd.commands.clone(), &cmd.name, &cmd.method, keyword),
        }
    }
    pub async fn get_result(&self, keyword: &String)->Option<(String, String)>{
        match self {
            Launcher::App(_) => None,
            Launcher::Web(_) => None,
            Launcher::Calc(_) => None,
            Launcher::BulkText(api) => api.get_result(keyword).await,
            Launcher::SystemCommand(_) => None,
        }
    }
    pub fn priority(&self)->u32{
        match self {
            Launcher::App(app) => app.priority,
            Launcher::Web(web) => web.priority,
            Launcher::Calc(calc) => calc.priority,
            Launcher::BulkText(api) => api.priority,
            Launcher::SystemCommand(cmd) => cmd.priority,
        }
    }
    pub fn alias(&self)->String{
        match self {
            Launcher::App(app) => app.alias.clone().unwrap_or_default(),
            Launcher::Web(web) => web.alias.clone().unwrap_or_default(),
            Launcher::Calc(calc) => calc.alias.clone().unwrap_or_default(),
            Launcher::BulkText(api) => api.alias.clone().unwrap_or_default(),
            Launcher::SystemCommand(cmd) => cmd.alias.clone().unwrap_or_default(),
        }
    }
    pub fn name(&self)->String{
        match self {
            Launcher::App(app) => app.name.clone(),
            Launcher::Web(web) => web.name.clone(),
            Launcher::Calc(calc) => calc.name.clone(),
            Launcher::BulkText(api) => api.name.clone(),
            Launcher::SystemCommand(cmd) => cmd.name.clone(),
        }
    }
    pub fn is_async(&self)->bool{
        match self {
            Launcher::App(app) => app.r#async.clone(),
            Launcher::Web(web) => web.r#async.clone(),
            Launcher::Calc(calc) => calc.r#async.clone(),
            Launcher::BulkText(api) => api.r#async.clone(),
            Launcher::SystemCommand(cmd) => cmd.r#async.clone(),
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

