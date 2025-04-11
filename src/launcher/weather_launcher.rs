use simd_json::{base::{ValueAsArray, ValueAsScalar}, derived::ValueObjectAccess};

#[derive(Clone, Debug)]
pub struct WeatherLauncher {
    pub location: String,
}
impl WeatherLauncher {
    pub async fn get_result(&self) -> Option<(String, String)> {
        let url = format!("https://wttr.in/{}?format=j2", self.location);
        let response = reqwest::get(url).await.ok()?.text().await.ok()?;

        // Convert the response to a mutable byte buffer
        let mut json_bytes = response.into_bytes(); // creates Vec<u8>

        let json: simd_json::OwnedValue = simd_json::to_owned_value(&mut json_bytes).ok()?;
        let current = json["current_condition"].as_array()?.get(0)?;
        let temp_c = current["temp_C"].as_str()?;

        let weather_desc = current["weatherDesc"]
            .as_array()?
            .get(0)?
            .get("value")?
            .as_str()?;
        return Some((temp_c.to_string(), weather_desc.to_string()));
    }
}
