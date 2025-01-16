use std::collections::HashMap;
use std::fs::{self, read_to_string};
use std::path::Path;
use regex::Regex;
use rayon::prelude::*;
use glob::Pattern;

use super::util::SherlockFlags;
use super::{Loader, util};
use util::{read_file, AppData, SherlockAlias};

impl Loader{
    pub fn load_applications(sherlock_flags: &SherlockFlags, app_config: &util::Config) -> HashMap<String, AppData> {
        let sherlock_ignore_path = sherlock_flags.ignore.clone();
        let sherlock_alias_path = sherlock_flags.alias.clone();

        let system_apps = "/usr/share/applications/";
        let mut ignore_apps: Vec<Pattern> = Default::default();

        let name_re = Regex::new(r"(?i)Name\s*=\s*(.*)\n").unwrap();
        let icon_re = Regex::new(r"(?i)Icon\s*=\s*(.*)\n").unwrap();
        let exec_re = Regex::new(r"(?i)Exec\s*=\s*(.*)\n").unwrap();
        let display_re = Regex::new(r"(?i)NoDisplay\s*=\s*(.*)\n").unwrap();
        let terminal_re = Regex::new(r"(?i)Terminal\s*=\s*(.*)\n").unwrap();
        let keywords_re = Regex::new(r"(?i)Keywords\s*=\s*(.*)\n").unwrap();

        let parse_field = |content: &str, regex: &Regex| {
            regex
                .captures(content)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
                .unwrap_or_default()
        };

        //Check if user has created sherlockignore file and parse ignores
        if Path::new(&sherlock_ignore_path).exists() {
            ignore_apps = read_to_string(sherlock_ignore_path)
                .unwrap() 
                .lines()
                .filter_map(|line| {
                    let line = line.to_lowercase();
                    Pattern::new(&line).ok() // Only include valid patterns
                })
            .collect();
        }        

        //Check if user has created sherlockalias file
        let sherlock_aliases:HashMap<String, SherlockAlias> = if Path::new(&sherlock_alias_path).exists(){
            match fs::read_to_string(sherlock_alias_path) {
                Ok(json_data) => match serde_json::from_str(&json_data) {
                    Ok(alias_map) => alias_map,
                    Err(e) => {
                        eprint!("Failed to parse alias file. {}", e);
                        HashMap::new()
                    }
                },
                Err(e) => {
                    eprint!("Failed to read alias file. {}", e);
                    HashMap::new()
                }
            }
        } else {
            HashMap::new()
        };
        println!("{:?}", sherlock_aliases);



        let dektop_files:Vec<_> = fs::read_dir(system_apps)
            .expect("Unable to read/access /usr/share/applications directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map(|ext| ext == "desktop").unwrap_or(false))
            .collect();

        let file_contents:Vec<String> = dektop_files
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
                let mut name = parse_field(&content, &name_re);
                if name.is_empty() || should_ignoe(&ignore_apps, &name){
                    return None; // Skip entries with empty names
                }

                let mut keywords = parse_field(&content, &keywords_re);
                let mut icon = parse_field(&content, &icon_re);

                // Construct the executable command
                let exec_path = parse_field(&content, &exec_re);
                let exec = if parse_field(&content, &terminal_re) == "true" {
                    if let Some(terminal) = &app_config.default_apps.terminal {
                        format!("{} {}", terminal, exec_path)
                    } else {
                        eprintln!("E5000 No terminal found!");
                        exec_path.to_string()
                    }
                } else {
                    exec_path.to_string()
                };



                // apply aliases
                if let Some(alias) = sherlock_aliases.get(&name){
                    if let Some(alias_name) = alias.name.as_ref() {
                        name = alias_name.to_string();
                    }
                    if let Some(alias_icon) = alias.icon.as_ref() {
                        icon = alias_icon.to_string();
                    }
                    if let Some(alias_keywords) = alias.keywords.as_ref() {
                        keywords = alias_keywords.to_string();
                    }
                    
                };

                let search_string = format!("{};{}", name, keywords);
                // Return the processed app data
                Some((name, AppData { icon, exec, search_string}))
            })
        .collect();
        apps
    }
}

fn should_ignoe(ignore_apps: &Vec<Pattern>, app: &String)->bool{
    let app_name = app.to_lowercase();
    ignore_apps.iter().any(|pattern| {
        pattern.matches(&app_name)
    })
}
