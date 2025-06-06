use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use gio::{glib::WeakRef, ListStore};
use gtk4::{
    prelude::{EntryExt, WidgetExt},
    ApplicationWindow,
};

use crate::{
    loader::pipe_loader::deserialize_pipe,
    ui::{search::SearchUiObj, tiles::Tile, util::SearchHandler},
    utils::errors::SherlockError,
};

pub struct SherlockAPI {
    pub window: Option<WeakRef<ApplicationWindow>>,
    pub search_ui: Option<WeakRef<SearchUiObj>>,
    pub search_handler: Option<SearchHandler>,
    pub errors: Option<WeakRef<ListStore>>,
}
impl SherlockAPI {
    pub fn new() -> Self {
        Self {
            window: None,
            search_ui: None,
            search_handler: None,
            errors: None,
        }
    }
    pub fn obfuscate(&self, visibility: bool) {
        if let Some(ui) = self.search_ui.as_ref().and_then(|ui| ui.upgrade()) {
            let imp = ui.imp();
            imp.search_bar.set_visibility(visibility);
        }
    }
    pub fn clear_results(&self) -> Option<()> {
        let handler = self.search_handler.as_ref()?;
        if let Some(model) = handler.model.as_ref().and_then(|s| s.upgrade()) {
            model.remove_all();
        }
        Some(())
    }
    pub fn show_raw(&self) -> Option<()> {
        let ui = self.search_ui.as_ref().and_then(|ui| ui.upgrade())?;
        let imp = ui.imp();
        let handler = self.search_handler.as_ref()?;
        if let Some(model) = handler.model.as_ref().and_then(|s| s.upgrade()) {
            model.remove_all();
        }
        imp.mode_title.set_visible(false);
        imp.mode_title.unparent();
        imp.all.set_visible(false);
        imp.status_bar.set_visible(false);
        Some(())
    }
    pub fn display_pipe(&self, content: &str) -> Option<()> {
        let handler = self.search_handler.as_ref()?;
        let model = handler.model.as_ref().and_then(|s| s.upgrade())?;
        handler.clear();

        let buf = content.as_bytes().to_vec();
        let parsed = deserialize_pipe(buf);
        let data = Tile::pipe_data(&parsed, "print");
        println!("{:?}", data.len());
        data.into_iter().for_each(|elem| {
            model.append(&elem);
        });
        Some(())
    }
    pub fn insert_msg(&self, error: SherlockError) -> Option<()> {
        let model = self.errors.as_ref().and_then(|tmp| tmp.upgrade())?;
        let (_, tiles) = Tile::error_tile(0, &vec![error], "⚠️", "WARNING");
        println!("{:?}", tiles);
        model.append(tiles.first()?);
        Some(())
    }
}
