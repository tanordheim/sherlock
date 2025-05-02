use std::collections::HashSet;

use gio::glib::WeakRef;
use gtk4::Box;

pub mod app_launcher;
pub mod audio_launcher;
pub mod bulk_text_launcher;
pub mod calc_launcher;
pub mod category_launcher;
pub mod clipboard_launcher;
pub mod event_launcher;
pub mod process_launcher;
pub mod system_cmd_launcher;
mod utils;
pub mod weather_launcher;
pub mod web_launcher;

use crate::{
    g_subclasses::sherlock_row::SherlockRow,
    ui::tiles::{util::AsyncLauncherTile, Tile},
};

use app_launcher::App;
use audio_launcher::MusicPlayerLauncher;
use bulk_text_launcher::BulkText;
use calc_launcher::Calculator;
use category_launcher::CategoryLauncher;
use clipboard_launcher::ClipboardLauncher;
use event_launcher::EventLauncher;
use process_launcher::ProcessLauncher;
use system_cmd_launcher::SystemCommand;
use weather_launcher::{WeatherData, WeatherLauncher};
use web_launcher::Web;

#[derive(Clone, Debug)]
pub enum LauncherType {
    CategoryLauncher(CategoryLauncher),
    App(App),
    Web(Web),
    Calc(Calculator),
    BulkText(BulkText),
    SystemCommand(SystemCommand),
    Clipboard((ClipboardLauncher, Calculator)),
    EventLauncher(EventLauncher),
    MusicPlayerLauncher(MusicPlayerLauncher),
    ProcessLauncher(ProcessLauncher),
    WeatherLauncher(WeatherLauncher),
    Empty,
}
/// # Launcher
/// ### Fields:
/// - **name:** Specifies the name of the launcher – such as a category e.g. `App Launcher`
/// - **alias:** Also referred to as `mode` – specifies the mode in which the launcher children should
/// be active in
/// - **tag_start:** Specifies the text displayed in a custom UI Label
/// - **tag_end:** Specifies the text displayed in a custom UI Label
/// - **method:** Specifies the action that should be executed on `row-should-activate` action
/// - **next_content:** Specifies the content to be displayed whenever method is `next`
/// - **priority:** Base priority all children inherit from. Children priority will be a combination
/// of this together with their execution counts and levenshtein similarity
/// - **r#async:** Specifies whether the tile should be loaded/executed asynchronously 
/// - **home:** Specifies whether the children should show on the `home` mode (empty
/// search entry & mode == `all`)
/// - **launcher_type:** Used to specify the kind of launcher and subsequently its children
/// - **shortcut:** Specifies whether the child tile should show `modekey + number` shortcuts
/// - **spawn_focus:** Specifies whether the tile should have focus whenever Sherlock launches
/// - **only_home:** Specifies whether the children should **only** show on the `home` mode (empty
/// search entry & mode == `all`)
#[derive(Clone, Debug)]
pub struct Launcher {
    pub name: Option<String>,
    pub alias: Option<String>,
    pub tag_start: Option<String>,
    pub tag_end: Option<String>,
    pub method: String,
    pub next_content: Option<String>,
    pub priority: u32,
    pub r#async: bool,
    pub home: bool,
    pub launcher_type: LauncherType,
    pub shortcut: bool,
    pub spawn_focus: bool,
    pub only_home: bool,
}

#[derive(Clone, Debug)]
pub struct ResultItem {
    pub priority: f32,
    pub row_item: SherlockRow,
    pub shortcut_holder: Option<WeakRef<Box>>,
}

impl Launcher {
    // TODO: tile method recreates already stored data...
    pub fn get_patch(&self, keyword: &str) -> Vec<ResultItem> {
        match &self.launcher_type {
            LauncherType::App(app) => Tile::app_tile(self, keyword, &app.apps),
            LauncherType::Calc(calc) => Tile::calc_tile(self, &calc, keyword),
            LauncherType::CategoryLauncher(ctg) => Tile::app_tile(self, keyword, &ctg.categories),
            LauncherType::Clipboard((clp, calc)) => {
                Tile::clipboard_tile(self, &clp, &calc, keyword)
            }
            LauncherType::EventLauncher(evl) => Tile::event_tile(self, keyword, evl),
            LauncherType::ProcessLauncher(proc) => Tile::process_tile(self, keyword, &proc),
            LauncherType::SystemCommand(cmd) => Tile::app_tile(self, keyword, &cmd.commands),
            LauncherType::Web(web) => Tile::web_tile(self, keyword, &web),

            _ => Vec::new(),
        }
    }
    pub fn get_execs(&self) -> Option<HashSet<String>> {
        // NOTE: make a function to check for exec changes in the caching algorithm
        match &self.launcher_type {
            LauncherType::App(app) => {
                let execs: HashSet<String> = app.apps.iter().map(|v| v.exec.to_string()).collect();
                Some(execs)
            }
            LauncherType::Web(web) => {
                let exec = format!("websearch-{}", web.engine);
                let execs: HashSet<String> = HashSet::from([(exec)]);
                Some(execs)
            }
            LauncherType::SystemCommand(cmd) => {
                let execs: HashSet<String> =
                    cmd.commands.iter().map(|v| v.exec.to_string()).collect();
                Some(execs)
            }
            LauncherType::CategoryLauncher(ctg) => {
                let execs: HashSet<String> =
                    ctg.categories.iter().map(|v| v.exec.to_string()).collect();
                Some(execs)
            }

            // None-Home Launchers
            LauncherType::Calc(_) => None,
            LauncherType::BulkText(_) => None,
            LauncherType::Clipboard(_) => None,
            LauncherType::EventLauncher(_) => None,
            _ => None,
        }
    }
    pub fn get_loader_widget(self, keyword: &str) -> Option<(AsyncLauncherTile, ResultItem)> {
        match self.launcher_type.clone() {
            LauncherType::BulkText(bulk_text) => {
                Tile::bulk_text_tile_loader(self, keyword, &bulk_text)
            }
            LauncherType::MusicPlayerLauncher(mpris) => Tile::mpris_tile(self, &mpris),
            LauncherType::WeatherLauncher(_) => Tile::weather_tile_loader(self),
            _ => None,
        }
    }
    pub async fn get_result(&self, keyword: &str) -> Option<(String, String, Option<String>)> {
        match &self.launcher_type {
            LauncherType::BulkText(bulk_text) => bulk_text.get_result(keyword).await,
            _ => None,
        }
    }
    pub async fn get_image(&self) -> Option<(gdk_pixbuf::Pixbuf, bool)> {
        match &self.launcher_type {
            LauncherType::MusicPlayerLauncher(mpis) => mpis.get_image().await,
            _ => None,
        }
    }
    pub async fn get_weather(&self) -> Option<(WeatherData, bool)> {
        match &self.launcher_type {
            LauncherType::WeatherLauncher(wtr) => wtr.get_result().await,
            _ => None,
        }
    }
}
