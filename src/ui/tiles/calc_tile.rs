use gtk4::{self, Box, Builder, Label, ListBoxRow};
use meval::eval_str;

use super::util::{get_builder, insert_attrs};
use super::Tile;

impl Tile {
    pub fn calc_tile(index: i32, equation: &str) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        match eval_str(equation) {
            Ok(result) => {
                let builder = get_builder("/dev/skxxtz/sherlock/ui/calc_tile.ui", index);
                builder.equation_holder.set_text(&equation);
                builder.result_holder.set_text(format!("= {}", result.to_string()).as_str());

                let result = result.to_string();

                let attrs: Vec<(&str, &str)> = vec![
                    ("method", "copy"),
                    ("result", result.as_str()),
                ];
                insert_attrs(&builder.attrs, attrs);

                results.push(builder.object);
            }
            _ => {}
        }

        (index, results)
    }
}
