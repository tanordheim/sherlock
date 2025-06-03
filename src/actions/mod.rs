use std::collections::HashMap;

use gio::glib::{object::IsA, variant::ToVariant};
use gtk4::{prelude::WidgetExt, Widget};
use teamslaunch::teamslaunch;
use util::{clear_cached_files, reset_app_counter};

use crate::{
    launcher::{audio_launcher::MusicPlayerLauncher, process_launcher::ProcessLauncher},
    loader::util::CounterReader,
};

pub mod applaunch;
pub mod commandlaunch;
pub mod teamslaunch;
pub mod util;
pub mod websearch;

pub fn execute_from_attrs<T: IsA<Widget>>(
    row: &T,
    attrs: &HashMap<String, String>,
    do_exit: Option<bool>,
) {
    //construct HashMap
    let attrs: HashMap<String, String> = attrs
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    if let Some(method) = attrs.get("method") {
        let mut exit = attrs.get("exit").map_or(true, |s| s == "true");

        match method.as_str() {
            "categories" => {
                exit = false;
                attrs.get("exec").map(|mode| {
                    let _ = row.activate_action("win.switch-mode", Some(&mode.to_variant()));
                    let _ = row.activate_action("win.clear-search", None);
                });
            }
            "app_launcher" => {
                let exec = attrs.get("exec").map_or("", |s| s.as_str());
                let term = attrs.get("term").map_or(false, |s| s.as_str() == "true");
                applaunch::applaunch(exec, term);
                increment(&exec);
            }
            "web_launcher" | "bookmarks" => {
                let engine = attrs.get("engine").map_or("plain", |s| s.as_str());
                let query = if let Some(query) = attrs.get("exec") {
                    query.as_str()
                } else if let Some(query) = attrs.get("keyword") {
                    let exec = format!("websearch-{}", engine);
                    increment(&exec);
                    query.as_str()
                } else {
                    ""
                };
                let _ = websearch::websearch(engine, query);
            }
            "command" => {
                let exec = attrs.get("exec").map_or("", |s| s.as_str());
                let keyword = attrs.get("keyword").map_or("", |s| s.as_str());
                if let Err(error) = commandlaunch::command_launch(exec, keyword) {
                    println!("{}", error);
                }
                increment(&exec);
            }
            "copy" => {
                if let Some(field) = attrs.get("field") {
                    if let Some(output) = attrs.get(field) {
                        let _ = util::copy_to_clipboard(output.as_str());
                    }
                } else if let Some(result) = attrs.get("result") {
                    let _ = util::copy_to_clipboard(result.as_str());
                }
            }
            "print" => {
                if let Some(field) = attrs.get("field") {
                    if let Some(output) = attrs.get(field) {
                        print!("{}", output);
                    }
                } else if let Some(result) = attrs.get("result") {
                    print!("{}", result);
                }
            }
            "teams_event" => {
                if let Some(meeting) = attrs.get("meeting_url") {
                    match teamslaunch(meeting) {
                        Ok(_) => {
                            let _ = row.activate_action("win.close", None);
                        }
                        Err(_) => {
                            let _ = row.activate_action(
                                "win.switch-page",
                                Some(&String::from("search-page->error-page").to_variant()),
                            );
                        }
                    }
                }
            }
            "emoji_picker" => {
                exit = false;
                let _ = row.activate_action("win.emoji-page", None);
                let _ = row.activate_action(
                    "win.switch-page",
                    Some(&String::from("search-page->emoji-page").to_variant()),
                );
            }
            "next" => {
                exit = false;
                let next_content = attrs
                    .get("next_content")
                    .map_or("No next_content provided...", |s| s.trim());

                let _ = row
                    .activate_action("win.add-page", Some(&next_content.to_string().to_variant()));
            }
            "play-pause" | "audio_sink" => {
                attrs
                    .get("player")
                    .map(|player| MusicPlayerLauncher::playpause(player));
            }
            "kill-process" => {
                let _ = attrs
                    .get("parent-pid")
                    .and_then(|p| p.parse::<i32>().ok())
                    .zip(attrs.get("child-pid").and_then(|c| c.parse::<i32>().ok()))
                    .map(|(ppid, cpid)| ProcessLauncher::kill((ppid, cpid)));
            }
            "debug" => {
                let exec = attrs.get("exec").map_or("", |s| s.as_str());
                match exec {
                    "show_errors" => {
                        let _result = row.activate_action(
                            "win.switch-page",
                            Some(&String::from("search-page->error-page").to_variant()),
                        );
                        increment("debug.show_errors");
                        exit = false;
                    }
                    "clear_cache" => {
                        let _result = clear_cached_files();
                        increment("debug.clear_cache");
                    }
                    "reset_counts" => {
                        let _result = reset_app_counter();
                        increment("debug.reset_counts");
                    }
                    _ => {}
                }
            }
            "clear_cache" => {
                let _result = clear_cached_files();
            }
            _ => {
                if let Some(out) = attrs.get("result") {
                    print!("{}", out);
                } else {
                    println!("Return method \"{}\" not recognized", method);
                }
            }
        }

        exit = do_exit.unwrap_or(exit);
        if exit {
            eval_close(row);
        }
    }
}
pub fn get_attrs_map(in_attrs: Vec<(&str, Option<&str>)>) -> HashMap<String, String> {
    in_attrs
        .into_iter()
        .filter_map(|(k, v)| {
            if let (k, Some(v)) = (k, v) {
                Some((k.to_string(), v.to_string()))
            } else {
                None
            }
        })
        .collect()
}
fn increment(key: &str) {
    if let Ok(count_reader) = CounterReader::new() {
        let _ = count_reader.increment(key);
    };
}
fn eval_close<T: IsA<Widget>>(row: &T) {
    let _ = row.activate_action("win.close", None);
}
