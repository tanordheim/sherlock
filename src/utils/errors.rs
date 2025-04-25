use std::{fmt::Debug, path::PathBuf};

#[derive(Clone, Debug)]
pub struct SherlockError {
    pub error: SherlockErrorType,
    pub traceback: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SherlockErrorType {
    // Environment
    EnvVarNotFoundError(String),

    // Filesystem - Files
    FileExistError(PathBuf),
    FileReadError(PathBuf),
    FileWriteError(PathBuf),
    FileParseError(PathBuf),
    FileRemoveError(PathBuf),

    // Filesystem - Directories
    DirReadError(String),
    DirCreateError(String),
    DirRemoveError(String),

    // Config & Flags
    ConfigError(Option<String>),
    FlagLoadError,

    // Resources
    ResourceParseError,
    ResourceLookupError(String),

    // Display / UI
    DisplayError,
    ClipboardError,

    // Regex / Parsing
    RegexError(String),

    // Commands
    CommandExecutionError(String),

    // DBus
    DBusConnectionError,
    DBusMessageConstructError(String),
    DBusMessageSendError(String),

    // Networking
    HttpRequestError(String),

    // Sockets
    SocketRemoveError(String),
    SocketConnectError(String),
    SocketWriteError(String),
}
fn variant_name<T: Debug>(e: &T) -> String {
    let full = format!("{:?}", e);
    full.split('(').next().unwrap_or(&full).to_string()
}
impl SherlockErrorType {
    pub fn get_message(&self) -> (String, String) {
        let message = match self {
            // Environment
            SherlockErrorType::EnvVarNotFoundError(var) => format!("Failed to unpack environment variable \"{}\"", var),

            // Filesystem - Files
            SherlockErrorType::FileExistError(file) => format!("File \"{}\" does not exist", file.to_string_lossy()),
            SherlockErrorType::FileReadError(file) => format!("Failed to read file \"{}\"", file.to_string_lossy()),
            SherlockErrorType::FileWriteError(file) => format!("Failed to write file \"{}\"", file.to_string_lossy()),
            SherlockErrorType::FileParseError(file) => format!("Failed to parse file \"{}\"", file.to_string_lossy()),
            SherlockErrorType::FileRemoveError(file) => format!("Failed to remove file \"{}\"", file.to_string_lossy()),
            

            // Filesystem - Directories
            SherlockErrorType::DirReadError(dir) => format!("Failed to read/access dir \"{}\"", dir),
            SherlockErrorType::DirCreateError(dir) => format!("Failed to create parent dir \"{}\"", dir),
            SherlockErrorType::DirRemoveError(dir) => format!("Failed to remove dir \"{}\"", dir),

            // Config & Flags
            SherlockErrorType::ConfigError(val) => {
                if let Some(v) = val {
                    format!("{}", v)
                } else {
                    "It should never come to this".to_string()
                }
            }
            SherlockErrorType::FlagLoadError => "Failed to load flags".to_string(),

            // Resources
            SherlockErrorType::ResourceParseError => "Failed to parse resources".to_string(),
            SherlockErrorType::ResourceLookupError(resource) => format!("Failed to find resource \"{}\"", resource),

            // Display / UI
            SherlockErrorType::DisplayError => "Could not connect to a display".to_string(),
            SherlockErrorType::ClipboardError => "Failed to get system clipboard".to_string(),

            // Regex / Parsing
            SherlockErrorType::RegexError(key) => format!("Failed to compile the regular expression for \"{}\"", key),

            // Commands
            SherlockErrorType::CommandExecutionError(cmd) => format!("Failed to execute command \"{}\"", cmd),

            // DBus
            SherlockErrorType::DBusConnectionError => "Failed to connect to system DBus".to_string(),
            SherlockErrorType::DBusMessageConstructError(message) => format!("Failed to construct Dbus message \"{}\"", message),
            SherlockErrorType::DBusMessageSendError(message) => format!("Failed to send Dbus message \"{}\"", message),

            // Networking
            SherlockErrorType::HttpRequestError(resource) => format!("Failed to get requested source \"{}\"", resource),

            // Sockets
            SherlockErrorType::SocketRemoveError(socket) => format!("Failed to close socket at location \"{}\"", socket),
            SherlockErrorType::SocketConnectError(socket) => format!("Failed to connect to socket at location \"{}\"", socket),
            SherlockErrorType::SocketWriteError(socket) => format!(
                "Failed to send message to socket at location \"{}\"",
                socket
            ),
        };
        (variant_name(self), message)
    }
}
