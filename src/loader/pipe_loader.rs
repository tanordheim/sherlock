use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::os::linux::fs::MetadataExt;

use gtk4::gdk::Display;
use gtk4::IconTheme;
use serde::Deserialize;

use crate::api::server::SherlockServer;
use crate::CONFIG;

use super::Loader;

impl Loader {
    pub fn load_pipe_args() -> Vec<u8> {
        if let Ok(metadata) = File::open("/dev/stdin").and_then(|f| f.metadata()) {
            // 0o020000 - Character device (e.g. TTY)
            // 0o170000 - octal mask to extract all file types
            if metadata.st_mode() & 0o170000 == 0o020000 {
                return vec![];
            }
        }
        let stdin = io::stdin();
        let mut buf = Vec::new();
        let _ = stdin.lock().read_to_end(&mut buf);
        return buf;
    }
}

pub fn deserialize_pipe(mut buf: Vec<u8>) -> Option<Vec<PipedElements>> {
    let mut data: Option<PipedData> = simd_json::from_slice(&mut buf).ok();
    if let Some(settings) = data.as_mut().and_then(|d| d.settings.take()) {
        if let Some(obfuscate) = settings.obfuscate {
            let _ = SherlockServer::send(format!(r#""obfuscate": {}"#, obfuscate));
        }
    }

    let config = CONFIG.get()?;

    match data.as_mut().and_then(|d| d.elements.take()) {
        Some(mut parsed_data) => {
            for i in parsed_data.iter_mut() {
                if i.field.is_none() {
                    i.field = config.behavior.field.clone();
                }
                if let Some(title) = &i.title {
                    let cleaned: String = title
                        .chars()
                        .filter(|&c| c.is_ascii() && (!c.is_control() || c == '\t' || c == '\n'))
                        .collect();
                    i.title = Some(cleaned);
                }
                if let Some(desc) = &i.description {
                    let cleaned: String = desc
                        .chars()
                        .filter(|&c| c.is_ascii() && (!c.is_control() || c == '\t' || c == '\n'))
                        .collect();
                    i.description = Some(cleaned);
                }
                if let Some(res) = &i.result {
                    let cleaned: String = res
                        .chars()
                        .filter(|&c| c.is_ascii() && (!c.is_control() || c == '\t' || c == '\n'))
                        .collect();
                    i.result = Some(cleaned);
                }
            }
            Some(parsed_data)
        }
        None if data.is_none() => {
            let icon_theme = IconTheme::for_display(Display::default().as_ref().unwrap());
            let mut result = Vec::new();
            let mut start = 0;
            while start < buf.len() {
                // Detect if the current byte sequence is a valid UTF-8 string
                let end = match buf[start..].iter().position(|&b| b == b'\n') {
                    Some(pos) => start + pos + 1,
                    None => buf.len(),
                };

                // Try to convert the current chunk to a UTF-8 string
                let chunk = &buf[start..end];

                // Check if the chunk contains valid UTF-8
                if let Ok(line) = std::str::from_utf8(chunk) {
                    // Treat it as a normal string (text line)
                    let mut raw_meta: Vec<&str> = line.split('\0').collect();
                    let name = raw_meta.remove(0).to_string();
                    let mut meta_data: HashMap<String, String> = raw_meta
                        .into_iter()
                        .filter_map(|s| {
                            let mut parts = s.split('\x1f');
                            match (parts.next(), parts.next()) {
                                (Some(k), Some(v)) => Some((k.to_string(), v.to_string())),
                                _ => None,
                            }
                        })
                        .collect();

                    let icons: Vec<String> = meta_data
                        .remove("icon")
                        .map(|i| {
                            i.split(',')
                                .filter(|name| icon_theme.has_icon(name))
                                .map(str::to_string)
                                .collect()
                        })
                        .unwrap_or_default();

                    result.push(PipedElements {
                        title: Some(name.clone()),
                        description: None,
                        result: Some(name),
                        icon: icons.get(0).cloned(),
                        icon_size: None,
                        binary: None,
                        method: None,
                        field: config.behavior.field.clone(),
                        hidden: None,
                        exit: true,
                    });
                } else {
                    // If it's not valid UTF-8, treat it as binary data
                    result.push(PipedElements {
                        title: None,
                        description: None,
                        result: None,
                        icon: None,
                        icon_size: None,
                        binary: Some(chunk.to_vec()),
                        field: config.behavior.field.clone(),
                        method: None,
                        hidden: None,
                        exit: true,
                    });
                }

                start = end;
            }

            Some(result)
        }
        _ => None,
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PipedElements {
    pub title: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub icon_size: Option<i32>,
    pub result: Option<String>,
    pub binary: Option<Vec<u8>>,
    pub method: Option<String>,
    pub field: Option<String>,
    pub hidden: Option<HashMap<String, String>>,
    pub exit: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PipedSettings {
    pub obfuscate: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PipedData {
    pub settings: Option<PipedSettings>,
    pub elements: Option<Vec<PipedElements>>,
}
