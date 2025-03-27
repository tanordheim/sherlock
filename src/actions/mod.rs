use std::collections::HashMap;
use std::process::exit;

use teamslaunch::teamslaunch;

use crate::{
    ui::{
        user::{display_next, display_raw},
        window::hide_window,
    },
    CONFIG,
};

pub mod applaunch;
pub mod commandlaunch;
pub mod util;
pub mod websearch;
pub mod teamslaunch;

pub fn execute_from_attrs(attrs: HashMap<String, String>) {
    if let Some(method) = attrs.get("method") {
        match method.as_str() {
            "app_launcher" => {
                let exec = attrs.get("exec").map_or("", |s| s.as_str());
                applaunch::applaunch(exec);
                eval_exit();
            }
            "web_launcher" => {
                let query = attrs.get("keyword").map_or("", |s| s.as_str());
                let engine = attrs.get("engine").map_or("", |s| s.as_str());
                let _ = websearch::websearch(engine, query);
                eval_exit();
            }
            "command" => {
                let exec = attrs.get("exec").map_or("", |s| s.as_str());
                let keyword = attrs.get("keyword").map_or("", |s| s.as_str());
                let _ = commandlaunch::command_launch(exec, keyword);
                eval_exit();
            }
            "copy" => {
                if let Some(result) = attrs.get("result") {
                    let _ = util::copy_to_clipboard(result.as_str());
                }
            }
            "teams_event" => {
                if let Some(meeting) = attrs.get("meeting_url") {
                    teamslaunch(meeting);
                }
                eval_exit();
            }
            "next" => {
                let next_content = attrs
                    .get("next_content")
                    .map_or("No next_content provided...", |s| s);
                display_next(next_content);
            }
            "display_raw" => {
                if let Some(next_content) = attrs.get("next_content") {
                    display_raw(next_content, false);
                }
            }
            _ => {
                if let Some(out) = attrs.get("text_content") {
                    print!("{}", out);
                }
                eval_exit();
            }
        }
    }
}

fn eval_exit() {
    if let Some(c) = CONFIG.get() {
        match c.behavior.daemonize {
            true => hide_window(true),
            false => exit(0),
        }
    }
}
