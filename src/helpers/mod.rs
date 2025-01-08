use gio::glib;
use gtk4::{prelude::*, CssProvider, Widget, Label, ListBox};
use std::collections::HashMap;
use std::fs::{self, read_to_string, File};
use std::env;
use std::io::{BufReader, Read};
use std::path::Path;
use regex::Regex;
use serde::Deserialize;
use gtk4::gdk::Display;
use rayon::prelude::*;

use crate::CONFIG;

pub mod config_loader;

#[derive(Deserialize, Clone, Debug)]
pub struct AppData{
    pub icon: String,
    pub exec: String,
}

pub fn load_resources(){
    let res_bytes = include_bytes!("../../resources.gresources");
    let resource = gio::Resource::from_data(&glib::Bytes::from_static(res_bytes))
        .expect("Failed to load resources in helpers.");
    gio::resources_register(&resource);
}

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/com/skxxtz/sherlock/main.css");
    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Cound not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn read_file(file_path: &str)->std::io::Result<String>{
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

pub fn get_applications() -> HashMap<String, AppData> {
    let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home/user"));
    let sherlock_ignore_path = format!("{}/.config/sherlock/sherlockignore", home_dir);

    //Check if user has created sherlockignore file
    let system_apps = "/usr/share/applications/";
    let mut ignore_apps: Vec<String> = Default::default();

    let name_re = Regex::new(r"(?i)Name\s*=\s*(.*)\n").unwrap();
    let icon_re = Regex::new(r"(?i)Icon\s*=\s*(.*)\n").unwrap();
    let exec_re = Regex::new(r"(?i)Exec\s*=\s*(.*)\n").unwrap();
    let display_re = Regex::new(r"(?i)NoDisplay\s*=\s*(.*)\n").unwrap();
    let terminal_re = Regex::new(r"(?i)Terminal\s*=\s*(.*)\n").unwrap();

    let parse_field = |content: &str, regex: &Regex| {
        regex
            .captures(content)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_default()
    };

    if Path::new(&sherlock_ignore_path).exists(){
        ignore_apps = read_to_string(sherlock_ignore_path).unwrap().lines().map(String::from).collect();
    }

    let files:Vec<_> = fs::read_dir(system_apps)
        .expect("Unable to read/access /usr/share/applications directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map(|ext| ext == "desktop").unwrap_or(false))
        .collect();
    {}
    let file_contents:Vec<String> = files
        .into_par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            let rpath = path.to_str()?;
            read_file(rpath).ok()
        }).collect();


    let apps: HashMap<String, AppData> = file_contents
        .into_par_iter()
        .filter_map(|content| {
            // Skip if "NoDisplay" field is set to "true"
            if parse_field(&content, &display_re) == "true" {
                return None;
            }

            // Extract fields
            let name = parse_field(&content, &name_re);
            if name.is_empty() || ignore_apps.contains(&name){
                return None; // Skip entries with empty names
            }

            let icon = parse_field(&content, &icon_re);
            let exec_path = parse_field(&content, &exec_re);

            // Construct the executable command
            let exec = if parse_field(&content, &terminal_re) == "true" {
                if let Some(terminal) = &CONFIG.defaults.terminal {
                    format!("{} {}", terminal, exec_path)
                } else {
                    eprintln!("E5000 No terminal found!");
                    exec_path.to_string()
                }
            } else {
                exec_path.to_string()
            };

            // Return the processed app data
            Some((name, AppData { icon, exec }))
        })
    .collect();
    apps
}


pub fn select_first_row(results: &ListBox){
    if let Some(first_row) = results.first_child(){
        if let Some(row) = first_row.downcast_ref::<gtk4::ListBoxRow>() {
            results.select_row(Some(row));
        } 
    }
}

pub fn read_from_label(label_obj:&Widget)->Option<(String, String)>{
    if let Some(label) = label_obj.downcast_ref::<Label>(){
        let text = label.text();
        let parts: Vec<&str> = text.split(" | ").collect();

        if parts.len() == 2 {
            return Some((parts[0].to_string(), parts[1].to_string()))
        }
    }
    return None
}
