use gio::glib::object::ObjectExt;
use gtk4::prelude::WidgetExt;
use meval::eval_str;

use super::util::TileBuilder;
use super::Tile;
use crate::{
    actions::{execute_from_attrs, get_attrs_map},
    launcher::{Launcher, ResultItem},
};

impl Tile {
    pub fn calc_tile(launcher: &Launcher, equation: &str, result: Option<f64>) -> Vec<ResultItem> {
        let result = if let Some(r) = result {
            r
        } else if let Ok(r) = eval_str(equation.trim()) {
            r
        } else {
            return vec![];
        };

        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/calc_tile.ui");
        builder.object.add_css_class("calc-tile");
        builder.object.set_spawn_focus(launcher.spawn_focus);
        builder.object.set_shortcut(launcher.shortcut);

        builder.equation_holder.set_text(&equation);
        builder
            .result_holder
            .set_text(format!("= {}", result.to_string()).as_str());

        let result = result.to_string();

        // Add action capabilities
        let attrs = get_attrs_map(vec![("method", &launcher.method), ("result", &result)]);
        builder
            .object
            .connect("row-should-activate", false, move |_row| {
                execute_from_attrs(&attrs);
                None
            });

        let shortcut_holder = match launcher.shortcut {
            true => builder.shortcut_holder,
            _ => None,
        };

        let res = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
            shortcut_holder,
        };

        vec![res]
    }
}
