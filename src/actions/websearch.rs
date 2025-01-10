use std::collections::HashMap;

pub fn websearch(engine: &str, query: &str) {
    let mut engines: HashMap<&str, &str> = Default::default();
    engines.insert("google", "https://www.google.com/search?q={}");
    engines.insert("bing", "https://www.bing.com/search?q={}");
    engines.insert("duckduckgo", "https://duckduckgo.com/?q={}");
    if let Some(url_template) = engines.get(engine) {
        let url = url_template.replace("{}", query);
        let _ = open::that(url);
    }
}

