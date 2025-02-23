use std::collections::HashMap;
use std::process::exit;

pub mod applaunch;
pub mod commandlaunch;
pub mod util;
pub mod websearch;

pub fn execute_from_attrs(attrs: HashMap<String, String>) {
    if let Some(method) = attrs.get("method") {
        match method.as_str() {
            "app_launcher" => {
                let exec = attrs.get("exec").expect("Missing field: exec");
                applaunch::applaunch(exec);
                exit(0);
            }
            "web_launcher" => {
                let query = attrs
                    .get("keyword")
                    .expect("Missing field: keyword (query)");
                let engine = attrs.get("engine").expect("Missing field: engine (query)");
                websearch::websearch(engine, query);
                exit(0);
            }
            "command" => {
                let exec = attrs.get("exec").expect("Missing field: exec");
                let keyword = attrs.get("keyword").expect("Missing field: keyword");
                let _ = commandlaunch::command_launch(exec, keyword);
                exit(0)
            }
            "copy" => {
                let string = attrs.get("result").expect("Missing field: result");
                let _ = util::copy_to_clipboard(string);
                exit(0)
            }
            _ => {
                eprint!("Invalid method: {}", method)
            }
        }
    }
}
