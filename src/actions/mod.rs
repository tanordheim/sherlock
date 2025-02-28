use std::collections::HashMap;
use std::process::exit;

use crate::ui::tiles::util::TextViewTileBuilder;
use crate::ui::util::show_stack_page;
use crate::APP_STATE;

pub mod applaunch;
pub mod commandlaunch;
pub mod util;
pub mod websearch;

pub fn execute_from_attrs(attrs: HashMap<String, String>) {
    if let Some(method) = attrs.get("method") {
        match method.as_str() {
            "app_launcher" => {
                let exec = attrs.get("exec").map_or("", |s| s.as_str());
                applaunch::applaunch(exec);
                exit(0);
            }
            "web_launcher" => {
                let query = attrs.get("keyword").map_or("", |s| s.as_str());
                let engine = attrs.get("engine").map_or("", |s| s.as_str());
                let _  = websearch::websearch(engine, query);
                exit(0);
            }
            "command" => {
                let exec = attrs.get("exec").map_or("", |s| s.as_str());
                let keyword = attrs.get("keyword").map_or("", |s| s.as_str());
                let _ = commandlaunch::command_launch(exec, keyword);
                exit(0)
            }
            "copy" => {
                if let Some(result) = attrs.get("result") {
                    let _ = util::copy_to_clipboard(result.as_str());
                }
            },
            "next" => {
                if let Some(next_content) = attrs.get("next_content") {
                    APP_STATE.with(|state|{
                        if let Some(ref state) = *state.borrow(){
                            let builder = TextViewTileBuilder::new("/dev/skxxtz/sherlock/ui/text_view_tile.ui");
                            builder.content.set_text(&next_content);

                            if let Some(stack) = &state.stack{
                                stack.add_named(&builder.object, Some("next-page"));
                                show_stack_page("next-page", Some(gtk4::StackTransitionType::SlideLeft));
                            }
                        }
                    });
                }
            }
            _ => {
                if let Some(out) = attrs.get("text_content"){
                    print!("{}", out);
                }
                exit(0)

            }
        }
    }
}
