use bytes::Bytes;
use gtk4::gdk_pixbuf::{Pixbuf, PixbufLoader};
use gtk4::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;

use dbus::{
    arg::{RefArg, Variant},
    blocking::{BlockingSender, Connection},
    Message,
};

use crate::loader::util::{SherlockError, SherlockErrorType};

#[derive(Debug, Clone)]
pub struct MusicPlayerLauncher {
    pub artist: String,
    pub _album_artist: String,
    pub title: String,
    pub _album: String,
    pub art: String,
    pub _url: String,
    pub player: String,
}
impl MusicPlayerLauncher {
    pub async fn get_image(&self) -> Option<(Pixbuf, bool)> {
        let loc = match &self.art.split("/").last() {
            Some(s) => s.to_string(),
            _ => return None,
        };
        let mut was_cached = true;
        let bytes = match MusicPlayerLauncher::read_cached_cover(&loc) {
            Ok(b) => b,
            Err(_) => {
                if self.art.starts_with("file"){
                    MusicPlayerLauncher::read_image_file(&self.art).ok()?
                } else {
                    let response = reqwest::get(&self.art).await.ok()?;
                    let bytes = response.bytes().await.ok()?;
                    let _ = MusicPlayerLauncher::cache_cover(&bytes, &loc);
                    was_cached = false;
                    bytes
                }
            }
        };

        let loader = PixbufLoader::new();
        loader.write(&bytes).ok()?;
        loader.close().ok()?;
        loader.pixbuf().and_then(|i| Some((i, was_cached)))
    }
    fn cache_cover(image: &Bytes, loc: &str) -> Result<(), SherlockError> {
        // Create dir and parents

        let home = env::var("HOME").map_err(|e| SherlockError {
            error: SherlockErrorType::EnvVarNotFoundError("HOME".to_string()),
            traceback: e.to_string(),
        })?;

        let home_dir = PathBuf::from(home);
        let path = home_dir.join(".sherlock/mpris-cache/").join(loc);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| SherlockError {
                error: SherlockErrorType::DirCreateError(".sherlock/mpris-cache/".to_string()),
                traceback: e.to_string(),
            })?;
        };

        let mut file = if path.exists() {
            File::open(&path)
        } else {
            File::create(&path)
        }
        .map_err(|e| SherlockError {
            error: SherlockErrorType::FileExistError(format!("{:?}", &path)),
            traceback: e.to_string(),
        })?;

        file.write_all(&image).map_err(|e| SherlockError {
            error: SherlockErrorType::FileExistError(format!("{:?}", &path)),
            traceback: e.to_string(),
        })?;
        // if file not exist, create and write it
        Ok(())
    }
    fn read_cached_cover(loc: &str) -> Result<Bytes, SherlockError> {
        let home = env::var("HOME").map_err(|e| SherlockError {
            error: SherlockErrorType::EnvVarNotFoundError("HOME".to_string()),
            traceback: e.to_string(),
        })?;
        let home_dir = PathBuf::from(home);
        let path = home_dir.join(".sherlock/mpris-cache/").join(loc);

        let mut file = File::open(path).map_err(|e| SherlockError {
            error: SherlockErrorType::FileExistError(loc.to_string()),
            traceback: e.to_string(),
        })?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).map_err(|e| SherlockError {
            error: SherlockErrorType::FileReadError(loc.to_string()),
            traceback: e.to_string(),
        })?;
        Ok(buffer.into())
    }
    fn read_image_file(loc: &str) -> Result<Bytes, SherlockError> {
        let path = PathBuf::from(loc.trim_start_matches("file://"));

        let mut file = File::open(path).map_err(|e| SherlockError {
            error: SherlockErrorType::FileExistError(loc.to_string()),
            traceback: e.to_string(),
        })?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).map_err(|e| SherlockError {
            error: SherlockErrorType::FileReadError(loc.to_string()),
            traceback: e.to_string(),
        })?;
        Ok(buffer.into())
    }
    pub fn playpause(player: &str) -> Result<(), SherlockError> {
        let conn = Connection::new_session().map_err(|e| SherlockError {
            error: SherlockErrorType::DBusConnectionError,
            traceback: e.to_string(),
        })?;
        let msg = Message::new_method_call(
            player,
            "/org/mpris/MediaPlayer2",
            "org.mpris.MediaPlayer2.Player",
            "PlayPause",
        )
        .map_err(|e| SherlockError {
            error: SherlockErrorType::DBusMessageConstructError(format!(
                "PlayPause for {}",
                player
            )),
            traceback: e.to_string(),
        })?;
        let _reply = conn
            .send_with_reply_and_block(msg, Duration::from_millis(500))
            .map_err(|e| SherlockError {
                error: SherlockErrorType::DBusMessageSendError(format!("PlayPause to {}", player)),
                traceback: e.to_string(),
            })?;
        Ok(())
    }
}

pub struct AudioLauncherFunctions {
    conn: Connection,
}

impl AudioLauncherFunctions {
    pub fn new() -> Option<Self> {
        let conn = Connection::new_session().ok()?;
        Some(AudioLauncherFunctions { conn })
    }
    pub fn get_current_player(&self) -> Option<String> {
        // Send a message to ListNames on the D-Bus to get all service names
        let msg = Message::new_method_call(
            "org.freedesktop.DBus",  // D-Bus service name
            "/org/freedesktop/DBus", // Object path
            "org.freedesktop.DBus",  // Interface
            "ListNames",             // Method to list service names
        )
        .ok()?;

        // Send the message and block for the reply
        let reply = self
            .conn
            .send_with_reply_and_block(msg, Duration::from_secs(1))
            .ok()?;

        // Process the reply and filter MPRIS services
        if let Some(services) = reply.get1::<Vec<String>>() {
            let mpris_services: Vec<String> = services
                .into_iter()
                .filter(|s| s.starts_with("org.mpris.MediaPlayer2."))
                .collect();

            // Return the first MPRIS service name found, if any
            return mpris_services.into_iter().next();
        }

        // If no MPRIS services were found, return None
        None
    }

    pub fn get_metadata(&self, service_name: &str) -> Option<MusicPlayerLauncher> {
        // The object path for the MPRIS service
        let object_path = "/org/mpris/MediaPlayer2";
        let interface_name = "org.freedesktop.DBus.Properties";

        // Send a message to get the "Metadata" property from the MPRIS service
        let msg = Message::new_method_call(service_name, object_path, interface_name, "Get")
            .ok()?
            .append1("org.mpris.MediaPlayer2.Player") // Interface
            .append1("Metadata"); // Property

        // Send the message and block for the reply
        let reply = self
            .conn
            .send_with_reply_and_block(msg, Duration::from_secs(1))
            .ok()?;
        let mut meta_data: HashMap<String, String> = HashMap::new();
        if let Some(variant) = reply.get1::<Variant<HashMap<String, Variant<Box<dyn RefArg>>>>>() {
            variant.0.into_iter().for_each(|(k, v)| {
                if let Some(s) = v.0.as_any().downcast_ref::<String>() {
                    meta_data.insert(k.clone(), s.clone());
                } else if let Some(s) = v.0.as_any().downcast_ref::<Vec<String>>() {
                    meta_data.insert(k.clone(), s.join(", "));
                }
            });
        }
        // println!("{:?}", meta_data);
        Some(MusicPlayerLauncher {
            artist: meta_data
                .get("xesam:artist")
                .map(|f| f.to_string())
                .unwrap_or_default(),
            _album_artist: meta_data
                .get("xesam:albumArtist")
                .map(|f| f.to_string())
                .unwrap_or_default(),
            title: meta_data
                .get("xesam:title")
                .map(|f| f.to_string())
                .unwrap_or_default(),
            _album: meta_data
                .get("xesam:album")
                .map(|f| f.to_string())
                .unwrap_or_default(),
            art: meta_data
                .get("mpris:artUrl")
                .map(|f| f.to_string())
                .unwrap_or_default(),
            _url: meta_data
                .get("xesam:url")
                .map(|f| f.to_string())
                .unwrap_or_default(),
            player: service_name.to_string(),
        })
    }
}
