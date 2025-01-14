use std::collections::HashMap;
use std::process::Command;

pub fn websearch(engine: &str, query: &str) {
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
        ("startpage", "https://www.startpage.com/sp/search?q={keyword}"),
    ]);
    let url_template = if let Some(url) = engines.get(engine) {
        url
    } else {
        engine
    };
    let url = url_template.replace("{keyword}", query);
    if let Err(e) = Command::new("sh")
        .arg("-c")
            .arg(format!("xdg-open '{}'", url)) // Linux
            .spawn()
    {
        eprintln!("Failed to open browser: {}", e);
    }
}
