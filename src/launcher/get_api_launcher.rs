use regex::Regex;
use tokio::time::{timeout, Duration};

#[derive(Clone, Debug)]
pub struct ApiGet{
    pub alias: Option<String>,
    pub method: String,
    pub uuid: String,
    pub name: String,
    pub url: String,
    pub title_key: String,
    pub body_key: String,
    pub icon: String,
    pub priority: u32,
    pub r#async: bool,
}

impl ApiGet {
    pub async fn get_result(&self, keyword: &String) -> Option<(String, String)> {
        let timeout_duration = Duration::new(2, 0);
        let url = self.url.replace("{}",keyword);

        let response = timeout(timeout_duration, surf::get(url)).await;

        match response {
            Ok(Ok(mut resp)) => {
                match resp.body_string().await {
                    Ok(raw_body) => {
                        let body = raw_body.replace(r#"\""#, "'"); 
                        println!("{}", body);
                        let title = extract_key(&body, &self.title_key);
                        let body_content = extract_key(&body, &self.body_key);
                        Some((title, body_content))
                    },
                    Err(e) => {
                        Some(("Failed to read response body.".to_string(), format!("Error: {}", e)))
                    }
                }
            },
            Ok(Err(e)) => {
                Some(("Network request failed.".to_string(), format!("Error: {}", e)))
            },
            Err(_) => {
                Some(("Network request timed out.".to_string(), "".to_string()))
            }
        }
    }
}
fn extract_key(response:&String, key:&String)->String{
    let re = Regex::new(key).unwrap();
    if let Some(caps) = re.captures(response) {
        let g1 = caps.get(1).map_or("", |m| m.as_str());
        return g1.to_string()
    } else {
        return format!("Pattern {} not found.", key);
    };


}
