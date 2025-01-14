use gtk4::{Label, ListBoxRow, TextView};

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
pub struct LauncherCommons {
    pub name: String,
    pub alias: Option<String>,
    pub method: String,
    pub priority: u32,
    pub r#async: bool
}

#[derive(Clone, Debug)]
pub enum Launcher{
    App {common: LauncherCommons, specific: App},
    Web {common: LauncherCommons, specific: Web},
    Calc {common: LauncherCommons, specific: Calc},
    BulkText {common: LauncherCommons, specific: BulkText},
    SystemCommand {common: LauncherCommons, specific: SystemCommand},
}
impl Launcher{
    pub fn get_loader_widget(&self, keyword: &String)-> Option<(ListBoxRow, Label, TextView)>{
        match self {
            Launcher::App {..} => None,
            Launcher::Web {..} => None,
            Launcher::Calc {..} => None,
            Launcher::BulkText {common:c, specific:s} => Tile::bulk_text_tile_loader(&c.name, &c.method, &s.icon, keyword),
            Launcher::SystemCommand {..} => None,
        }
        
    }
    fn get_patch(&self, index:i32, keyword: &String)->(i32, Vec<ListBoxRow>){
        match self {
            Launcher::App {common: c, specific: s} => Tile::app_tile(index, s.apps.clone(), &c.name, &c.method, keyword),
            Launcher::Web {common: c, specific: s} => Tile::web_tile(&c.name, &c.method, &s.icon, &s.engine, index, keyword),
            Launcher::Calc {common: c, specific: _} => Tile::calc_tile(index, keyword, &c.method),
            Launcher::BulkText {common: c, specific: s} => Tile::bulk_text_tile(&c.name, &c.method, &s.icon, index, keyword),
            Launcher::SystemCommand {common: c, specific: s} => Tile::app_tile(index, s.commands.clone(), &c.name, &c.method, keyword),
        }
    }
    pub async fn get_result(&self, keyword: &String)->Option<(String, String)>{
        match self {
            Launcher::App {..} => None,
            Launcher::Web {..} => None,
            Launcher::Calc {..} => None,
            Launcher::BulkText {common: _, specific: s} => s.get_result(keyword).await,
            Launcher::SystemCommand {..} => None,
        }
    }
    pub fn priority(&self)->u32{
        match self {
            Launcher::App {common: c, specific: _} => c.priority,
            Launcher::Web {common: c, specific: _} => c.priority,
            Launcher::Calc {common: c, specific: _} =>c.priority,
            Launcher::BulkText {common: c, specific: _} => c.priority,
            Launcher::SystemCommand {common: c, specific: _} => c.priority,
        }
    }
    pub fn alias(&self)->String{
        match self {
            Launcher::App {common: c, specific: _} => c.alias.clone().unwrap_or_default(),
            Launcher::Web {common: c, specific: _} => c.alias.clone().unwrap_or_default(),
            Launcher::Calc {common: c, specific: _} => c.alias.clone().unwrap_or_default(),
            Launcher::BulkText {common: c, specific: _} => c.alias.clone().unwrap_or_default(),
            Launcher::SystemCommand {common: c, specific: _} => c.alias.clone().unwrap_or_default(),
        }
    }
    pub fn name(&self)->String{
        match self {
            Launcher::App {common: c, specific: _} => c.name.clone(),
            Launcher::Web {common: c, specific: _} => c.name.clone(),
            Launcher::Calc {common: c, specific: _} => c.name.clone(),
            Launcher::BulkText {common: c, specific: _} => c.name.clone(),
            Launcher::SystemCommand {common: c, specific: _} => c.name.clone(),
        }
    }
    pub fn is_async(&self)->bool{
        match self {
            Launcher::App {common: c, specific: _} => c.r#async.clone(),
            Launcher::Web {common: c, specific: _} => c.r#async.clone(),
            Launcher::Calc {common: c, specific: _} => c.r#async.clone(),
            Launcher::BulkText {common: c, specific: _} => c.r#async.clone(),
            Launcher::SystemCommand {common: c, specific: _} => c.r#async.clone(),
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

