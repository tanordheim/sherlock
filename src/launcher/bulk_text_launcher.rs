use std::process::Stdio;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::process::Command;
use tokio::time::{timeout, Duration};

#[derive(Clone, Debug)]
pub struct BulkText {
    pub icon: String,
    pub exec: String,
    pub args: String,
}

impl BulkText {
    pub async fn get_result(&self, keyword: &String) -> Option<(String, String)> {
        let a = self.args.replace("{keyword}", &keyword);
        let args = a.split(" ");
        let mut cmd = Command::new(&self.exec);
        cmd.args(args);
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Spawn the child process
        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(e) => {
                return Some((
                    "Failed to execute script.".to_string(),
                    format!("Error: {}", e),
                ));
            }
        };

        // Set up a timeout for the process
        let result = timeout(Duration::from_secs(2), async {
            let stdout = child.stdout.take();
            let mut stdout_content = String::new();
            let stderr_content = String::new();

            // If stdout is present, read asynchronously
            if let Some(stdout) = stdout {
                let mut reader = BufReader::new(stdout);
                reader.read_to_string(&mut stdout_content).await.ok();
            }

            // Wait for the child process to finish
            let status = child.wait().await;
            (status, stdout_content, stderr_content)
        })
        .await;

        // Handle the result
        match result {
            Ok((Ok(status), stdout, _stderr)) => {
                if status.success() {
                    let mut string_output =
                        String::from_utf8_lossy(stdout.as_bytes()).replace("#SHERLOCK_TITLE:", "");
                    string_output = string_output.replace("\\n", "\n");
                    string_output = string_output.replace('"', "");

                    let mut parts = string_output.split("#SHERLOCK_BODY:");
                    let mut title = parts.next().unwrap_or(keyword).trim();
                    let body = parts.next().unwrap_or("").trim();

                    if title.is_empty() {
                        title = keyword;
                    }

                    Some((title.to_string(), body.to_string()))
                } else {
                    Some((
                        "Script executed but returned an error.".to_string(),
                        format!("Status: {:?}", status),
                    ))
                }
            }
            Ok((Err(_), _, _)) => {
                let _ = child.kill().await; // Kill the process if it fails
                Some((
                    "Failed to execute script.".to_string(),
                    "Error occurred while running the process.".to_string(),
                ))
            }
            Err(_) => {
                let _ = child.kill().await; // Kill the process on timeout
                Some((
                    "Failed to execute script.".to_string(),
                    "Timeout exceeded.".to_string(),
                ))
            }
        }
    }
}
