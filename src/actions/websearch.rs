use std::collections::HashMap;
use std::process::Command;

use super::commandlaunch::command_launch;
use crate::loader::application_loader::{get_applications_dir, get_desktop_files};
use crate::utils::{
    errors::{SherlockError, SherlockErrorType},
    files::read_lines,
};

pub fn websearch(engine: &str, query: &str) -> Result<(), SherlockError> {
    let engines: HashMap<&str, &str> = HashMap::from([
        ("google", "https://www.google.com/search?q={keyword}"),
        ("bing", "https://www.bing.com/search?q={keyword}"),
        ("duckduckgo", "https://duckduckgo.com/?q={keyword}"),
        ("yahoo", "https://search.yahoo.com/search?p={keyword}"),
        ("baidu", "https://www.baidu.com/s?wd={keyword}"),
        ("yandex", "https://yandex.com/search/?text={keyword}"),
        ("ask", "https://www.ask.com/web?q={keyword}"),
        ("ecosia", "https://www.ecosia.org/search?q={keyword}"),
        ("qwant", "https://www.qwant.com/?q={keyword}"),
        (
            "startpage",
            "https://www.startpage.com/sp/search?q={keyword}",
        ),
        ("plain", "{keyword}"),
    ]);
    let url_template = if let Some(url) = engines.get(engine) {
        url
    } else {
        engine
    };

    // Find default browser desktop file
    let output = Command::new("xdg-settings")
        .arg("get")
        .arg("default-web-browser")
        .output()
        .map_err(|e| SherlockError {
            error: SherlockErrorType::EnvVarNotFoundError(String::from("default browser")),
            traceback: e.to_string(),
        })?;

    let desktop_file: String = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        return Err(SherlockError {
            error: SherlockErrorType::EnvVarNotFoundError("default browser".to_string()),
            traceback: String::new(),
        });
    };
    let desktop_dirs = get_applications_dir();
    let desktop_files = get_desktop_files(desktop_dirs);
    let browser_file = desktop_files
        .iter()
        .find(|f| f.ends_with(&desktop_file))
        .ok_or_else(|| SherlockError {
            error: SherlockErrorType::EnvVarNotFoundError("default browser".to_string()),
            traceback: String::new(),
        })?;
    // read default browser desktop file
    let mut browser = read_lines(browser_file)
        .map_err(|e| SherlockError {
            error: SherlockErrorType::FileReadError(browser_file.clone()),
            traceback: e.to_string(),
        })?
        .filter_map(Result::ok)
        .find(|line| line.starts_with("Exec="))
        .and_then(|line| line.strip_prefix("Exec=").map(|l| l.to_string()))
        .ok_or_else(|| SherlockError {
            error: SherlockErrorType::FileParseError(browser_file.clone()),
            traceback: String::new(),
        })?;

    let url = url_template.replace("{keyword}", &query.replace(" ", "+"));
    let command = if browser.contains("%u") {
        browser.replace("%u", &url)
    } else {
        browser.push_str(&format!(" {}", url));
        browser
    };
    command_launch(&command, "")
}
