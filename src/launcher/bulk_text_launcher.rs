use tokio::process::Command;
use tokio::time::{timeout, Duration};

#[derive(Clone, Debug)]
pub struct BulkText{
    pub alias: Option<String>,
    pub method: String,
    pub name: String,
    pub icon: String,
    pub priority: u32,
    pub r#async: bool,
    pub exec: String,
    pub args: String,
    pub whitespace: String,
}

impl BulkText {
    pub async fn get_result(&self, keyword: &String) -> Option<(String, String)> {
        let cleaned_keyword = keyword.replace(" ", self.whitespace.as_str());

        let a = self.args.replace("{}", &cleaned_keyword);
        let args = a.split(" ");
        let mut cmd = Command::new(&self.exec);
        cmd.args(args);

        // Timeout after 2 seconds
        let output = match timeout(Duration::from_secs(2), cmd.output()).await {
            Ok(Ok(value)) => value, 
            Ok(Err(e)) => return Some(("Failed to execute script.".to_string(), format!("Error: {}", e))),
            Err(_) => return Some(("Failed to execute script.".to_string(), "Timeout exceeded.".to_string())), // Timeout error
        };

        let mut string_output = String::from_utf8_lossy(&output.stdout).replace("#SHERLOCK_TITLE:", "");
        string_output = string_output.replace("\\n", "\n");
        string_output = string_output.replace('"', "");

        let mut parts = string_output.split("#SHERLOCK_BODY:");
        let mut title = parts.next().unwrap_or(keyword).trim();
        let body = parts.next().unwrap_or("").trim();

        if title.is_empty() {
            title = keyword;
        }

        Some((title.to_string(), body.to_string()))
    }}
