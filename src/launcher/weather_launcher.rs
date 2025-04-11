use gio::glib::subclass::interface;
use regex::Regex;
use simd_json::base::{ValueAsArray, ValueAsScalar};
use std::collections::HashSet;

use crate::CONFIG;

#[derive(Clone, Debug)]
pub struct WeatherLauncher {
    pub location: String,
}
impl WeatherLauncher {
    pub async fn get_result(&self) -> Option<(String, String, String)> {
        let config = CONFIG.get()?;
        let url = format!("https://wttr.in/{}?format=j2", self.location);

        let response = reqwest::get(url).await.ok()?.text().await.ok()?;
        let mut response_bytes = response.into_bytes();
        let json: simd_json::OwnedValue = simd_json::to_owned_value(&mut response_bytes).ok()?;
        let current_condition = json["current_condition"].as_array()?.get(0)?;

        // Parse Temperature
        let temp = match config.units.temperatures.as_str() {
            "t" | "T" => format!("{}°F", current_condition["temp_F"].as_str()?),
            _ => format!("{}°C", current_condition["temp_C"].as_str()?),
        };

        // Parse Icon
        let code = current_condition["weatherCode"].as_str()?;
        let icon = WeatherLauncher::match_weather_code(code);

        // Parse wind dir
        let wind_deg = current_condition["winddirDegree"]
            .as_str()?
            .parse::<f32>()
            .ok()?;
        let sector_size: f32 = 45.0;
        let index = ((wind_deg + sector_size / 2.0) / sector_size).floor() as usize % 8;
        let win_dirs = ["↑", "↗", "→", "↘", "↓", "↙", "←", "↖"];
        let wind_dir = win_dirs.get(index)?;

        // Parse wind speed
        let imperials: HashSet<&str> = HashSet::from([
            "inches", "inch", "in",
            "feet", "foot", "ft",
            "yards", "yard", "yd",
            "miles", "mile", "mi"
        ]);
        let wind = if imperials.contains(config.units.lengths.to_lowercase().as_str()){
            let speed = current_condition["windspeedMiles"].as_str()?;
            format!("{}{}mph", wind_dir, speed)
        } else {
            let speed = current_condition["windspeedKmph"].as_str()?;
            format!("{}{}km/h", wind_dir, speed)
        };

        return Some((temp, icon, wind));
    }
    fn match_weather_code(code: &str) -> String {
        let icon = match code {
            "113" => "weather-clear",
            "116" => "weather-few-clouds",
            "119" | "122" => "weather-many-clouds",
            "143" | "248" | "260" => "weather-mist",
            "176" | "263" | "299" | "305" | "353" | "356" => "weather-showers",
            "179" | "362" | "365" | "374" => "weather-freezing-scattered-rain-storm",
            "182" | "185" | "281" | "284" | "311" | "314" | "317" | "350" | "377" => {
                "weather-freezing-scattered-rain"
            }
            "200" | "302" | "308" | "359" | "386" | "389" => "weather-storm",
            "227" | "320" => "weather-snow-scattered-day",
            "230" | "329" | "332" | "338" => "weather-snow-storm",
            "323" | "326" | "335" | "368" | "371" | "392" | "395" => "weather-snow-scattered-storm",
            "266" | "293" | "296" => "weather-showers-scattered",
            _ => "weather-none-available",
        };
        String::from(icon)
    }
}
