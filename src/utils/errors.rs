use std::path::PathBuf;

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
impl SherlockErrorType {
    pub fn get_message(&self) -> (String, String) {
        match self {
            // Environment
            SherlockErrorType::EnvVarNotFoundError(var) => (
                "EnvVarNotFoundError".to_string(),
                format!("Failed to unpack environment variable \"{}\"", var),
            ),

            // Filesystem - Files
            SherlockErrorType::FileExistError(file) => (
                "FileExistError".to_string(),
                format!("File \"{}\" does not exist", file.to_string_lossy()),
            ),
            SherlockErrorType::FileReadError(file) => (
                "FileReadError".to_string(),
                format!("Failed to read file \"{}\"", file.to_string_lossy()),
            ),
            SherlockErrorType::FileWriteError(file) => (
                "FileWriteError".to_string(),
                format!("Failed to write file \"{}\"", file.to_string_lossy()),
            ),
            SherlockErrorType::FileParseError(file) => (
                "FileParseError".to_string(),
                format!("Failed to parse file \"{}\"", file.to_string_lossy()),
            ),
            SherlockErrorType::FileRemoveError(file) => (
                "FileRemoveError".to_string(),
                format!("Failed to remove file \"{}\"", file.to_string_lossy()),
            ),

            // Filesystem - Directories
            SherlockErrorType::DirReadError(dir) => (
                "DirReadError".to_string(),
                format!("Failed to read/access dir \"{}\"", dir),
            ),
            SherlockErrorType::DirCreateError(dir) => (
                "DirCreateError".to_string(),
                format!("Failed to create parent dir \"{}\"", dir),
            ),
            SherlockErrorType::DirRemoveError(dir) => (
                "DirRemoveError".to_string(),
                format!("Failed to remove dir \"{}\"", dir),
            ),

            // Config & Flags
            SherlockErrorType::ConfigError(val) => {
                let message = if let Some(v) = val {
                    format!("{}", v)
                } else {
                    "It should never come to this".to_string()
                };
                ("ConfigError".to_string(), message)
            }
            SherlockErrorType::FlagLoadError => (
                "FlagLoadError".to_string(),
                "Failed to load flags".to_string(),
            ),

            // Resources
            SherlockErrorType::ResourceParseError => (
                "ResourceParseError".to_string(),
                "Failed to parse resources".to_string(),
            ),
            SherlockErrorType::ResourceLookupError(resource) => (
                "ResourceLookupError".to_string(),
                format!("Failed to find resource \"{}\"", resource),
            ),

            // Display / UI
            SherlockErrorType::DisplayError => (
                "DisplayError".to_string(),
                "Could not connect to a display".to_string(),
            ),
            SherlockErrorType::ClipboardError => (
                "ClipboardError".to_string(),
                "Failed to get system clipboard".to_string(),
            ),

            // Regex / Parsing
            SherlockErrorType::RegexError(key) => (
                "RegexError".to_string(),
                format!("Failed to compile the regular expression for \"{}\"", key),
            ),

            // Commands
            SherlockErrorType::CommandExecutionError(cmd) => (
                "CommandExecutionError".to_string(),
                format!("Failed to execute command \"{}\"", cmd),
            ),

            // DBus
            SherlockErrorType::DBusConnectionError => (
                "DBusConnectionError".to_string(),
                "Failed to connect to system DBus".to_string(),
            ),
            SherlockErrorType::DBusMessageConstructError(message) => (
                "DBusMessageConstructError".to_string(),
                format!("Failed to construct Dbus message \"{}\"", message),
            ),
            SherlockErrorType::DBusMessageSendError(message) => (
                "DBusMessageSendError".to_string(),
                format!("Failed to send Dbus message \"{}\"", message),
            ),

            // Networking
            SherlockErrorType::HttpRequestError(resource) => (
                "HttpRequestError".to_string(),
                format!("Failed to get requested source \"{}\"", resource),
            ),

            // Sockets
            SherlockErrorType::SocketRemoveError(socket) => (
                "SocketRemoveError".to_string(),
                format!("Failed to close socket at location \"{}\"", socket),
            ),
            SherlockErrorType::SocketConnectError(socket) => (
                "SocketConnectError".to_string(),
                format!("Failed to connect to socket at location \"{}\"", socket),
            ),
            SherlockErrorType::SocketWriteError(socket) => (
                "SoecktWriteError".to_string(),
                format!(
                    "Failed to send message to socket at location \"{}\"",
                    socket
                ),
            ),
        }
    }
}
