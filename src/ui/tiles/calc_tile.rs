use gtk4::ListBoxRow;
use meval::eval_str;

use super::util::{get_builder, insert_attrs};
use super::Tile;
use crate::launcher::Launcher;

impl Tile {
    pub fn calc_tile(launcher: &Launcher, index: i32, equation: &str, result: Option<f64>) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        let result = if let Some(r) = result {
            r
        } else if let Ok(r) = eval_str(equation){
            r
        } else {
            return (index, results);
        };


        let builder = get_builder("/dev/skxxtz/sherlock/ui/calc_tile.ui", index, true);
        builder.equation_holder.set_text(&equation);
        builder
            .result_holder
            .set_text(format!("= {}", result.to_string()).as_str());

        let result = result.to_string();

        let attrs: Vec<(&str, &str)> =
            vec![("method", &launcher.method), ("result", result.as_str())];
        insert_attrs(&builder.attrs, attrs);

        results.push(builder.object);

        (index, results)
    }
}
