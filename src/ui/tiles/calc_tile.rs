use gtk4::prelude::WidgetExt;
use meval::eval_str;

use super::util::TileBuilder;
use super::Tile;
use crate::launcher::{Launcher, ResultItem};

impl Tile {
    pub fn calc_tile(launcher: &Launcher, equation: &str, result: Option<f64>) -> Vec<ResultItem> {
        let result = if let Some(r) = result {
            r
        } else if let Ok(r) = eval_str(equation.trim()) {
            if r.to_string().as_str() != equation.trim() {
                r
            } else {
                return vec![];
            }
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
        builder.add_default_attrs(Some(&launcher.method), Some(&result), None, None, None);

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
