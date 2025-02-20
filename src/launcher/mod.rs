use gtk4::{Label, ListBoxRow};

pub mod app_launcher;
pub mod bulk_text_launcher;
pub mod calc_launcher;
pub mod system_cmd_launcher;
pub mod web_launcher;
pub mod clipboard_launcher;

use crate::{loader::util::Config, ui::tiles::Tile};

use app_launcher::App;
use bulk_text_launcher::BulkText;
use calc_launcher::Calc;
use system_cmd_launcher::SystemCommand;
use web_launcher::Web;
use clipboard_launcher::Clp;

#[derive(Clone, Debug)]
pub enum LauncherType {
    App(App),
    Web(Web),
    Calc(Calc),
    BulkText(BulkText),
    SystemCommand(SystemCommand),
    Clipboard(Clp),
    Empty,
}

#[derive(Clone, Debug)]
pub struct Launcher {
    pub name: String,
    pub alias: Option<String>,
    pub method: String,
    pub priority: u32,
    pub r#async: bool,
    pub home: bool,
    pub launcher_type: LauncherType,
}
impl Launcher {
    // TODO: tile method recreates already stored data...
    pub fn get_patch(
        &self,
        index: i32,
        keyword: &String,
        app_config: &Config,
    ) -> (i32, Vec<ListBoxRow>) {
        match &self.launcher_type {
            LauncherType::App(app) => Tile::app_tile(
                index,
                app.apps.clone(),
                &self.name,
                &self.method,
                keyword,
                app_config,
            ),
            LauncherType::Web(web) => Tile::web_tile(
                &self.name,
                &self.method,
                &web.icon,
                &web.engine,
                index,
                keyword,
            ),
            LauncherType::Calc(_) => Tile::calc_tile(index, keyword, &self.method),
            LauncherType::BulkText(bulk_text) => {
                Tile::bulk_text_tile(&self.name, &self.method, &bulk_text.icon, index, keyword)
            }
            LauncherType::SystemCommand(cmd) => Tile::app_tile(
                index,
                cmd.commands.clone(),
                &self.name,
                &self.method,
                keyword,
                app_config,
            ),
            LauncherType::Clipboard(clp) => {
                Tile::clipboard_tile(index, &clp.clipboard_content)
            },

            _ => (index, Vec::new()),
        }
    }
    pub fn get_loader_widget(&self, keyword: &String) -> Option<(ListBoxRow, Label, Label)> {
        match &self.launcher_type {
            LauncherType::BulkText(bulk_text) => {
                Tile::bulk_text_tile_loader(&self.name, &self.method, &bulk_text.icon, keyword)
            }
            _ => None,
        }
    }
    pub async fn get_result(&self, keyword: &String) -> Option<(String, String)> {
        match &self.launcher_type {
            LauncherType::BulkText(bulk_text) => bulk_text.get_result(keyword).await,
            _ => None,
        }
    }
}

pub fn construct_tiles(
    keyword: &String,
    launchers: &[Launcher],
    mode: &String,
    app_config: &Config,
) -> Vec<ListBoxRow> {
    let mut widgets = Vec::new();
    let mut index: i32 = 0;
    let sel_mode = mode.trim();
    for launcher in launchers.iter() {
        let alias = launcher.alias.as_deref().unwrap_or("all");

        if launcher.priority == 0 && alias != sel_mode {
            continue;
        }

        if alias == sel_mode || sel_mode == "all" {
            let (returned_index, result) = launcher.get_patch(index, keyword, app_config);
            index = returned_index;
            widgets.extend(result);
        }
    }
    widgets
}
