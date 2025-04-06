use super::util::TileBuilder;
use super::Tile;
use crate::launcher::{calc_launcher::Calculator, Launcher, ResultItem};
use gtk4::prelude::WidgetExt;
use meval::eval_str;
use std::collections::HashSet;

impl Tile {
    pub fn calc_tile(
        launcher: &Launcher,
        calc_launcher: &Calculator,
        keyword: &str,
        just_print: Option<f64>,
    ) -> Vec<ResultItem> {
        let capabilities: HashSet<&str> = match &calc_launcher.capabilities {
            Some(c) => c.iter().map(|s| s.as_str()).collect(),
            _ => HashSet::from(["calc.math", "calc.measurement"]),
        };
        let mut result: Option<String> = None;

        if capabilities.contains("calc.math") {
            if let Some(r) = just_print {
                result = Some(format!("= {}", r.to_string()));
            } else if let Ok(r) = eval_str(keyword.trim()) {
                if r.to_string().as_str() != keyword.trim() {
                    result = Some(format!("= {}", r.to_string()));
                }
            }
        }

        if capabilities.contains("calc.measurement") && result.is_none() {
            if let Some(r) = calc_launcher.measurement(&keyword) {
                result = Some(r.to_string());
            }
        }

        if let Some(r) = result {
            let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/calc_tile.ui");
            builder.object.add_css_class("calc-tile");
            builder.object.set_spawn_focus(launcher.spawn_focus);
            builder.object.set_shortcut(launcher.shortcut);

            builder.equation_holder.set_text(&keyword);
            builder.result_holder.set_text(&r);

            let result = r.to_string();
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
        } else {
            return vec![];
        }
    }
}
