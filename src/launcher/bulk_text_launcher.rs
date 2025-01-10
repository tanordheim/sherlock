use tokio::process::Command;

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

        let output = match cmd.output().await {
            Ok(value) => value,
            Err(e) => return Some(("Failed to execute script.".to_string(), format!("Error: {}", e)))
        };
    
        let string_output = String::from_utf8_lossy(&output.stdout).replace("#SHERLOCK_TITLE:", "");
        let mut parts = string_output.split("#SHERLOCK_BODY:");
        let title = parts.next().unwrap_or(keyword);
        let body = parts.next().unwrap_or("");


        

        Some((title.trim_matches('"').to_string(), body.trim_matches('"').to_string()))
    }
}
