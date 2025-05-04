use lz4_flex::block::decompress_size_prepended;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use crate::loader::util::AppData;
use crate::utils::errors::{SherlockError, SherlockErrorType};

#[derive(Clone, Debug)]
pub struct BookmarkLauncher {
    pub bookmarks: HashSet<AppData>,
}
impl BookmarkLauncher {
    pub fn find_bookmarks(browser: &str, prio: f32) -> Result<HashSet<AppData>, SherlockError> {
        match browser {
            "zen" | "zen-browser" | "/opt/zen-browser-bin/zen-bin %u" => BookmarkParser::zen(prio),
            _ => {
                // @BaxoPlenty you can check here what this ↑ should be.
                println!("{:?}", browser);
                BookmarkParser::zen(prio)
            }
        }
    }
}

struct BookmarkParser;
impl BookmarkParser {
    fn zen(prio: f32) -> Result<HashSet<AppData>, SherlockError> {
        let path = "/home/basti/.zen/c8yxxptw.Default (release)/bookmarkbackups/bookmarks-2025-05-04_67_ZUtc1iq1cN2vI-qhCjIXNoSsj74LZxBh6mN54Kpbcj4=.jsonlz4";

        let data = fs::read(path).map_err(|e| SherlockError {
            error: SherlockErrorType::FileReadError(PathBuf::from(path)),
            traceback: format!("{}:{}\n{}", file!(), line!(), e.to_string()),
        })?;

        if &data[..8] != b"mozLz40\0" {
            return Err(SherlockError {
                error: SherlockErrorType::FileReadError(PathBuf::from(path)),
                traceback: format!("{}:{}\nInvalid JSONLZ4 header", file!(), line!()),
            });
        }

        let decompressed = decompress_size_prepended(&data[8..]).map_err(|e| SherlockError {
            error: SherlockErrorType::FileReadError(PathBuf::from(path)),
            traceback: format!("{}:{}\n{}", file!(), line!(), e.to_string()),
        })?;

        let json_value: Value =
            serde_json::from_slice(&decompressed).map_err(|e| SherlockError {
                error: SherlockErrorType::FileParseError(PathBuf::from(path)),
                traceback: format!("{}:{}\n{}", file!(), line!(), e.to_string()),
            })?;
        let mut bookmarks = HashSet::new();
        if let Some(children) = json_value["children"].as_array() {
            for folder in children.iter().skip(1) {
                extract_bookmarks(&folder, &mut bookmarks, prio);
            }
        }

        fn deserialize_bookmark(bookmark: &Value) -> Option<(String, String)> {
            if let (Some(title), Some(url)) = (bookmark["title"].as_str(), bookmark["uri"].as_str())
            {
                return Some((title.to_string(), url.to_string()));
            }
            None
        }

        fn extract_bookmarks(value: &serde_json::Value, out: &mut HashSet<AppData>, prio: f32) {
            if let Some(children) = value["children"].as_array() {
                for child in children {
                    if let Some((title, url)) = deserialize_bookmark(child) {
                        if !title.is_empty() {
                            let bookmark = AppData {
                                name: title.to_string(),
                                icon: None,
                                icon_class: None,
                                exec: url,
                                search_string: title,
                                tag_start: None,
                                tag_end: None,
                                desktop_file: None,
                                priority: prio,
                            };
                            out.insert(bookmark);
                        }
                    } else {
                        extract_bookmarks(child, out, prio);
                    }
                }
            }
        }
        Ok(bookmarks)
    }
}
