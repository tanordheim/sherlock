use std::collections::HashSet;
use regex::Regex;

use crate::CONFIG;

#[derive(Clone, Debug)]
pub struct WeatherLauncher {
    pub location: String,
}
impl WeatherLauncher {
    pub async fn get_result(&self) -> Option<(String, String, String)> {
        let url = format!("https://wttr.in/{}?format=%w--+%C--+%t", self.location);
        let pattern = r"^(.)(.*)km\/h--(.*)--\s(.*)°C$";
        let re = Regex::new(&pattern).ok()?;

        let response = reqwest::get(url).await.ok()?.text().await.ok()?;
        let matches = re.captures(&response)?;
        let wind_dir = matches.get(1)?.as_str();
        let wind_speed = matches.get(2)?.as_str().parse::<i32>().ok()?;
        let condition_raw = matches.get(3)?.as_str().trim();
        let temp_raw = matches.get(4)?.as_str().parse::<i32>().ok()?;

        let config = CONFIG.get()?;
        let temp_format = match config.units.temperatures.as_str() {
            "f" | "F" => {
                let farenheit = (temp_raw as f32 * 9.0 / 5.0) + 32.0;
                format!("{:.0}°F", farenheit)
        
            },
            _ => format!("{}°C", temp_raw)
        };
        let imperial_lengths: HashSet<&str> = HashSet::from([
            "inches", "inch", "in",
            "feet", "foot", "ft",
            "yards", "yard", "yd",
            "miles", "mile", "mi"
        ]);
        let wind_speed_format = if imperial_lengths.contains(config.units.lengths.to_lowercase().as_str()) {
            let mph = wind_speed as f32 * 0.621371;
            format!("{} {:.0}mph", wind_dir, mph)

        } else {
            format!("{} {:.0}km/h", wind_dir, wind_speed)
        };
        println!("{} - {}", condition_raw, wind_speed_format);
        let condition_format = WeatherLauncher::match_weather_code(condition_raw);
        let full_format = format!("{}  {}", condition_raw, wind_speed_format);


        return Some((temp_format, condition_format, full_format));
    }
    fn match_weather_code(code: &str) -> String {
        let icon_name = match code.to_lowercase().as_str() {
            "sunny" | "clear" => "weather-clear",
            "partly cloudy" => "weather-few-clouds",
            "cloudy" => "weather-many-clouds",
            "very cloudy" => "weather-many-clouds",
            "fog" => "weather-mist",
            "light showers" => "weather-showers",
            "light sleet showers" => "weather-freezing-scattered-rain-storm",
            "light sleet" => "weather-freezing-scattered-rain",
            "thundery showers" => "weather-storm",
            "light snow" => "weather-snow-scattered-day",
            "heavy snow" => "weather-snow-storm",
            "heavy snow showers" => "weather-snow-scattered-storm",
            "heavy showers" => "weather-showers",
            "heavy rain" => "weather-storm",
            "light rain" => "weather-showers-scattered",
            "light snow showers" => "weather-snow-scattered-storm",
            "thundery heavy rain" => "weather-storm",
            "thundery snow showers" => "weather-snow-scattered-storm",
            _ => "weather-none-available",
        };
        String::from(icon_name)
    }
}
