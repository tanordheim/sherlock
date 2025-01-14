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
    pub r#async: bool,
    pub home: bool,
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
    pub fn common(&self)-> &LauncherCommons {
        match self {
            Launcher::App { common, .. } => common,
            Launcher::Web { common, .. } => common,
            Launcher::Calc { common, .. } => common,
            Launcher::BulkText { common, .. } => common,
            Launcher::SystemCommand { common, .. } => common,
        }
    }
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
    pub fn name(&self)->String{
        self.common().name.clone()
    }
    pub fn alias(&self)->String{
        self.common().alias.clone().unwrap_or_default()
    }
    pub fn priority(&self)->u32{
        self.common().priority
    }
    pub fn is_async(&self)->bool{
        self.common().r#async
    }
    pub fn home(&self)->bool{
        self.common().home
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

