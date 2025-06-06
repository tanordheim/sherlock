use std::collections::HashMap;

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use gio::{glib::WeakRef, ListStore};
use gtk4::{
    prelude::{EntryExt, GtkWindowExt, WidgetExt},
    ApplicationWindow,
};

use crate::{
    actions::execute_from_attrs,
    loader::{pipe_loader::deserialize_pipe, util::JsonCache},
    ui::{
        search::SearchUiObj,
        tiles::Tile,
        util::{SearchHandler, SherlockAction, SherlockCounter},
    },
    utils::errors::SherlockError,
    CONFIG,
};

use super::call::ApiCall;

pub struct SherlockAPI {
    pub window: Option<WeakRef<ApplicationWindow>>,
    pub search_ui: Option<WeakRef<SearchUiObj>>,
    pub search_handler: Option<SearchHandler>,
    pub errors: Option<WeakRef<ListStore>>,
    pub awaiting: Vec<ApiCall>,
}
impl SherlockAPI {
    pub fn new() -> Self {
        Self {
            window: None,
            search_ui: None,
            search_handler: None,
            errors: None,
            awaiting: vec![],
        }
    }

    pub fn apply_action(&mut self, api_call: ApiCall) {
        self.clear_queue();
        if self.match_action(&api_call).is_none() {
            self.awaiting.push(api_call);
        }
    }
    pub fn clear_queue(&mut self) -> Option<()> {
        let mut awaiting = std::mem::take(&mut self.awaiting);
        self.awaiting = awaiting
            .drain(..)
            .filter(|api_call| self.match_action(api_call).is_some())
            .collect();
        Some(())
    }

    pub fn match_action(&mut self, api_call: &ApiCall) -> Option<()> {
        match api_call {
            ApiCall::Obfuscate(vis) => self.obfuscate(*vis),
            ApiCall::Clear => self.clear_results(),
            ApiCall::DisplayPipe(pipe) => self.display_pipe(pipe),
            ApiCall::SherlockError(err) => self.insert_msg(err),
            ApiCall::InputOnly => self.show_raw(),
            ApiCall::Show => self.open(),
            ApiCall::ClearAwaiting => self.clear_queue(),
        }
    }
    pub fn open(&self) -> Option<()> {
        let window = self.window.as_ref().and_then(|win| win.upgrade())?;
        let start_count = SherlockCounter::new()
            .and_then(|counter| counter.increment())
            .unwrap_or(0);

        let config = CONFIG.get()?;

        // parse sherlock actions
        let actions: Vec<SherlockAction> =
            JsonCache::read(&config.files.actions).unwrap_or_default();
        // activate sherlock actions
        actions
            .into_iter()
            .filter(|action| start_count % action.on == 0)
            .for_each(|action| {
                let attrs: HashMap<String, String> =
                    HashMap::from([(String::from("method"), action.action)]);
                execute_from_attrs(&window, &attrs, None);
            });
        window.present();
        Some(())
    }
    pub fn obfuscate(&self, vis: bool) -> Option<()> {
        let ui = self.search_ui.as_ref().and_then(|ui| ui.upgrade())?;
        let imp = ui.imp();
        imp.search_bar.set_visibility(vis);
        Some(())
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
        let parsed = deserialize_pipe(buf)?;
        let data = Tile::pipe_data(&parsed, "print");
        data.into_iter().for_each(|elem| {
            model.append(&elem);
        });
        Some(())
    }
    pub fn insert_msg(&self, error: &SherlockError) -> Option<()> {
        let model = self.errors.as_ref().and_then(|tmp| tmp.upgrade())?;
        let (_, tiles) = Tile::error_tile(0, &vec![error], "⚠️", "WARNING");
        model.append(tiles.first()?);
        Some(())
    }
}
