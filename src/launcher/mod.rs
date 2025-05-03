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

use crate::{g_subclasses::sherlock_row::SherlockRow, loader::util::RawLauncher, ui::tiles::Tile};

use app_launcher::AppLauncher;
use audio_launcher::MusicPlayerLauncher;
use bulk_text_launcher::BulkTextLauncher;
use calc_launcher::CalculatorLauncher;
use category_launcher::CategoryLauncher;
use clipboard_launcher::ClipboardLauncher;
use event_launcher::EventLauncher;
use process_launcher::ProcessLauncher;
use system_cmd_launcher::CommandLauncher;
use weather_launcher::{WeatherData, WeatherLauncher};
use web_launcher::WebLauncher;

#[derive(Clone, Debug)]
pub enum LauncherType {
    Category(CategoryLauncher),
    App(AppLauncher),
    Web(WebLauncher),
    Calc(CalculatorLauncher),
    BulkText(BulkTextLauncher),
    Command(CommandLauncher),
    Clipboard((ClipboardLauncher, CalculatorLauncher)),
    Event(EventLauncher),
    MusicPlayer(MusicPlayerLauncher),
    Process(ProcessLauncher),
    Weather(WeatherLauncher),
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
impl Launcher {
    pub fn from_raw(raw: RawLauncher, method: String, launcher_type: LauncherType) -> Self {
        Self {
            name: raw.name,
            alias: raw.alias,
            tag_start: raw.tag_start,
            tag_end: raw.tag_end,
            method,
            next_content: raw.next_content,
            priority: raw.priority as u32,
            r#async: raw.r#async,
            home: raw.home,
            only_home: raw.only_home,
            launcher_type,
            shortcut: raw.shortcut,
            spawn_focus: raw.spawn_focus,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResultItem {
    pub row_item: SherlockRow,
    pub shortcut_holder: Option<WeakRef<Box>>,
}

impl Launcher {
    // TODO: tile method recreates already stored data...
    pub fn get_patch(&self, keyword: &str) -> Vec<ResultItem> {
        match &self.launcher_type {
            LauncherType::App(app) => Tile::app_tile(self, keyword, &app.apps),
            LauncherType::Calc(calc) => Tile::calc_tile(self, &calc),
            LauncherType::Category(ctg) => Tile::app_tile(self, keyword, &ctg.categories),
            LauncherType::Clipboard((clp, calc)) => {
                Tile::clipboard_tile(self, &clp, &calc, keyword)
            }
            LauncherType::Event(evl) => Tile::event_tile(self, keyword, evl),
            LauncherType::Process(proc) => Tile::process_tile(self, keyword, &proc),
            LauncherType::Command(cmd) => Tile::app_tile(self, keyword, &cmd.commands),
            LauncherType::Web(web) => Tile::web_tile(self, keyword, &web),

            // Async tiles
            LauncherType::BulkText(bulk_text) => Tile::bulk_text_tile(self, keyword, &bulk_text),
            LauncherType::MusicPlayer(mpris) => Tile::mpris_tile(self, &mpris),
            LauncherType::Weather(_) => Tile::weather_tile_loader(self),
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
            LauncherType::Command(cmd) => {
                let execs: HashSet<String> =
                    cmd.commands.iter().map(|v| v.exec.to_string()).collect();
                Some(execs)
            }
            LauncherType::Category(ctg) => {
                let execs: HashSet<String> =
                    ctg.categories.iter().map(|v| v.exec.to_string()).collect();
                Some(execs)
            }

            // None-Home Launchers
            LauncherType::Calc(_) => None,
            LauncherType::BulkText(_) => None,
            LauncherType::Clipboard(_) => None,
            LauncherType::Event(_) => None,
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
            LauncherType::MusicPlayer(mpis) => mpis.get_image().await,
            _ => None,
        }
    }
    pub async fn get_weather(&self) -> Option<(WeatherData, bool)> {
        match &self.launcher_type {
            LauncherType::Weather(wtr) => wtr.get_result().await,
            _ => None,
        }
    }
}
