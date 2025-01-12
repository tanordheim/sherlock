use std::collections::HashMap;
use std::process::exit;

pub mod websearch;
pub mod applaunch;
pub mod commandlaunch;
pub mod util;

pub fn execute_from_attrs(attrs: HashMap<String, String>) {
    if let Some(method) = attrs.get("method") {
        match method.as_str() {
            "app" => {
                let exec = attrs.get("exec").expect("Missing field: exec");
                applaunch::applaunch(exec);
                exit(0);
            }
            "web" => {
                let query = attrs
                    .get("keyword")
                    .expect("Missing field: keyword (query)");
                let engine = attrs.get("engine").expect("Missing field: engine (query)");
                websearch::websearch(engine, query);
                exit(0);
            },
            "command" => {
                let exec = attrs.get("exec").expect("Missing field: exec");
                let keyword = attrs.get("keyword").expect("Missing field: keyword");
                commandlaunch::command_launch(exec, keyword);
                exit(0)
            },
            "calc" => {
                let string = attrs.get("result").expect("Missing field: result");
                util::copy_to_clipboard(string);
            },
            "bulk_text" => {
                let string = attrs.get("result").expect("Missing field: result");
                util::copy_to_clipboard(string);
            }
            _ => {
                eprint!("Invalid method: {}", method)
            }
        }
    }
}

