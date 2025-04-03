use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::os::linux::fs::MetadataExt;

use serde::Deserialize;

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
        return buf
    }
}

pub fn deserialize_pipe(mut buf: Vec<u8>) -> Vec<PipeData> {
    let data: Option<Vec<PipeData>> = simd_json::from_slice(&mut buf).ok();
    
    match data {
        Some(parsed_data) => parsed_data,
        None => {
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
                    let cleaned_line = line.replace('\0', "");
                    let cleaned_line = if !cleaned_line.is_empty() {
                        Some(cleaned_line.trim().to_string())
                    } else {
                        None
                    };
                    result.push(PipeData {
                        title: cleaned_line.clone(),
                        description: None,
                        result: cleaned_line,
                        icon: None,
                        binary: None,
                        method: None,
                        hidden: None,
                    });
                } else {
                    // If it's not valid UTF-8, treat it as binary data
                    result.push(PipeData {
                        title: None,
                        description: None,
                        result: None,
                        icon: None,
                        binary: Some(chunk.to_vec()),
                        method: None,
                        hidden: None,
                    });
                }

                start = end;
            }

            println!("{:?}", result);
            result
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PipeData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub result: Option<String>,
    pub binary: Option<Vec<u8>>,
    pub method: Option<String>,
    pub hidden: Option<HashMap<String,String>>
}
