use std::collections::HashMap;
use std::fs::{self, read_to_string};
use std::path::Path;
use regex::Regex;
use rayon::prelude::*;
use glob::Pattern;

use super::util::{SherlockError, SherlockFlags};
use super::{Loader, util};
use util::{read_file, AppData, SherlockAlias};

impl Loader{
    pub fn load_applications(sherlock_flags: &SherlockFlags, app_config: &util::Config) -> Result<HashMap<String, AppData>, SherlockError> {
        let sherlock_ignore_path = sherlock_flags.ignore.clone();
        let sherlock_alias_path = sherlock_flags.alias.clone();

        let system_apps = "/usr/share/applications/";
        let mut ignore_apps: Vec<Pattern> = Default::default();
        let mut aliases: HashMap<String, SherlockAlias> = Default::default();

        let (name_re, icon_re, exec_re, display_re, terminal_re, keywords_re) = get_regex_patterns()
            .map_err(|e| return e)?;

        let parse_field = |content: &str, regex: &Regex| {
            regex.captures(content)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
                .unwrap_or_default()
        };

        // Handle user specified ignore file
        if Path::new(&sherlock_ignore_path).exists() {
            ignore_apps = read_to_string(&sherlock_ignore_path)
                .map_err(|e| SherlockError {
                    name: "File Read Error".to_string(),
                    message: format!("Failed to read the file at '{}'", sherlock_ignore_path),
                    traceback: e.to_string(),
                })?
            .lines()
                .filter_map(|line| {
                    let line = line.to_lowercase();
                    Pattern::new(&line).ok()
                }).collect::<Vec<Pattern>>();
        }

        // Handle user specified alias file
        if Path::new(&sherlock_alias_path).exists() {
            let json_data = read_to_string(&sherlock_alias_path)
                .map_err(|e| SherlockError {
                    name: "File Read Error".to_string(),
                    message: format!("Failed to read the file at '{}'", sherlock_alias_path),
                    traceback: e.to_string(),
                })?;
            aliases = serde_json::from_str(&json_data)
                .map_err(|e| SherlockError {
                    name: "File Parse Error".to_string(),
                    message: format!("Failed to parse '{}' as valid json", sherlock_alias_path),
                    traceback: e.to_string(),
                })?
        }


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
                    format!("{} {}", &app_config.default_apps.terminal, exec_path)
                } else {
                    exec_path.to_string()
                };



                // apply aliases
                if let Some(alias) = aliases.get(&name){
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
        Ok(apps)
    }
}

fn should_ignoe(ignore_apps: &Vec<Pattern>, app: &String)->bool{
    let app_name = app.to_lowercase();
    ignore_apps.iter().any(|pattern| {
        pattern.matches(&app_name)
    })
}

fn get_regex_patterns() -> Result<(Regex, Regex, Regex, Regex, Regex, Regex), SherlockError> {
    let name_re = Regex::new(r"(?i)Name\s*=\s*(.*)\n")
        .map_err(|e| SherlockError {
            name: "Regex Compilation Error".to_string(),
            message: format!("Failed to compile the regular expression for 'Name'"),
            traceback: e.to_string(),
        })?;

    let icon_re = Regex::new(r"(?i)Icon\s*=\s*(.*)\n")
        .map_err(|e| SherlockError {
            name: "Regex Compilation Error".to_string(),
            message: format!("Failed to compile the regular expression for 'Icon'"),
            traceback: e.to_string(),
        })?;

    let exec_re = Regex::new(r"(?i)Exec\s*=\s*(.*)\n")
        .map_err(|e| SherlockError {
            name: "Regex Compilation Error".to_string(),
            message: format!("Failed to compile the regular expression for 'Exec'"),
            traceback: e.to_string(),
        })?;

    let display_re = Regex::new(r"(?i)NoDisplay\s*=\s*(.*)\n")
        .map_err(|e| SherlockError {
            name: "Regex Compilation Error".to_string(),
            message: format!("Failed to compile the regular expression for 'NoDisplay'"),
            traceback: e.to_string(),
        })?;

    let terminal_re = Regex::new(r"(?i)Terminal\s*=\s*(.*)\n")
        .map_err(|e| SherlockError {
            name: "Regex Compilation Error".to_string(),
            message: format!("Failed to compile the regular expression for 'Terminal'"),
            traceback: e.to_string(),
        })?;

    let keywords_re = Regex::new(r"(?i)Keywords\s*=\s*(.*)\n")
        .map_err(|e| SherlockError {
            name: "Regex Compilation Error".to_string(),
            message: format!("Failed to compile the regular expression for 'Keywords'"),
            traceback: e.to_string(),
        })?;

    // Return the tuple of compiled Regexes
    Ok((name_re, icon_re, exec_re, display_re, terminal_re, keywords_re))
}

