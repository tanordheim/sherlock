use gtk4::{self, Box, Builder, Label, ListBoxRow};
use meval::eval_str;

use super::util::insert_attrs;
use super::Tile;

impl Tile {
    pub fn calc_tile(index: i32, equation: &str) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        match eval_str(equation) {
            Ok(result) => {
                let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/calc_tile.ui");

                let holder: ListBoxRow = builder.object("holder").unwrap();
                let attr_holder: Box = builder.object("attrs-holder").unwrap();

                let equation_holder: Label = builder.object("equation-holder").unwrap();
                let result_holder: Label = builder.object("result-holder").unwrap();

                equation_holder.set_text(&equation);
                result_holder.set_text(format!("= {}", result.to_string()).as_str());

                let attrs: Vec<String> = vec![
                    format!("{} | {}", "method", "copy"),
                    format!("{} | {}", "result", result),
                ];
                insert_attrs(&attr_holder, attrs);

                results.push(holder);
            }
            _ => {}
        }

        (index, results)
    }
}
