use std::collections::HashMap;
use std::process::Command;

pub fn websearch(engine: &str, query: &str) {
    let mut engines: HashMap<&str, &str> = Default::default();
    engines.insert("google", "https://www.google.com/search?q={}");
    engines.insert("bing", "https://www.bing.com/search?q={}");
    engines.insert("duckduckgo", "https://duckduckgo.com/?q={}");

    if let Some(url_template) = engines.get(engine) {
        let url = url_template.replace("{}", query);
        if let Err(e) = Command::new("sh")
            .arg("-c")
                .arg(format!("xdg-open '{}' &", url)) // Linux
                .spawn()
        {
            eprintln!("Failed to open browser: {}", e);
        }
    }
}
