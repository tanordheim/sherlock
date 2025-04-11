use simd_json::{base::{ValueAsArray, ValueAsScalar}, derived::ValueObjectAccess};

use crate::CONFIG;

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

        let config = CONFIG.get()?;
        let temp = match config.units.temperatures.as_str() {
            "f" | "F" => {
                let tmp = current["temp_F"].as_str()?;
                format!("{}° F", tmp)
            }
            "c" | "C" => {
                let tmp = current["temp_C"].as_str()?;
                format!("{}° C", tmp)
            }
            _ => return None
        };

        let weather_desc = current["weatherDesc"]
            .as_array()?
            .get(0)?
            .get("value")?
            .as_str()?;
        return Some((temp, weather_desc.to_string()));
    }
}
