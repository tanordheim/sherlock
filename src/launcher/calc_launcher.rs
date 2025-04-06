use crate::CONFIG;
use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Calculator {
    pub capabilities: Option<HashSet<String>>,
}
impl Calculator {
    pub fn measurement(&self, keyword: &str) -> Option<String> {
        let full_pattern = r"(?i)(\d+(?:\.\d+)?)\s*([a-zA-Z]+)\s*(in|to)\s*([a-zA-Z]+)";
        let full_re = Regex::new(full_pattern).unwrap();
        if let Some(caps) = full_re.captures(keyword) {
            let value: f32 = caps[1].parse().ok()?;
            let from = caps[2].to_lowercase();
            let to = caps[4].to_lowercase();

            let (factor_from, _) = self.match_unit(&from)?;
            let (factor_to, name) = self.match_unit(&to)?;

            let base = self.to_basis(factor_from, value);
            let res = self.to_unit(factor_to, base);
            let postfix = if res == 1.0 { "" } else { "s" };
            return Some(format!("= {:.2} {}{}", res, name, postfix));
        }
        // Support for partial ones
        let part_pattern = r"(?i)(\d+(?:\.\d+)?)\s*([a-zA-Z]+)";
        let part_re = Regex::new(part_pattern).unwrap();
        if let Some(caps) = part_re.captures(keyword) {
            let config = CONFIG.get()?;
            let value: f32 = caps[1].parse().ok()?;
            let from = caps[2].to_lowercase();
            let to = config.units.lengths.to_lowercase();

            let (factor_from, _) = self.match_unit(&from)?;
            let (factor_to, name) = self.match_unit(&to)?;

            let base = self.to_basis(factor_from, value);
            let res = self.to_unit(factor_to, base);
            let postfix = if res == 1.0 { "" } else { "s" };
            return Some(format!("= {:.2} {}{}", res, name, postfix));
        }
        None
    }
    fn to_basis(&self, factor: f32, value: f32) -> f32 {
        value * factor
    }
    fn to_unit(&self, factor: f32, value: f32) -> f32 {
        value / factor
    }
    fn match_unit(&self, unit: &str) -> Option<(f32, String)> {
        match unit.to_lowercase().as_str() {
            // Metric units
            "kilometers" | "kilometer" | "kilos" | "km" => {
                Some((Measurement::KILOMETER, String::from("Kilometer")))
            }
            "meters" | "meter" | "m" => Some((1.0, String::from("Meter"))),
            "centimeters" | "centimeter" | "cm" | "cents" => {
                Some((Measurement::CENTIMETER, String::from("Centimeter")))
            }
            "millimeters" | "millimeter" | "mm" => {
                Some((Measurement::MILLIMETER, String::from("Millimeter")))
            }
            "micrometers" | "micrometer" | "um" | "µm" => {
                Some((Measurement::MICROMETER, String::from("Micrometer")))
            }

            // Imperial units
            "inches" | "inch" | "in" => Some((Measurement::INCH, String::from("Inch"))),
            "feet" | "foot" | "ft" => Some((Measurement::FEET, String::from("Feet"))),
            "yards" | "yard" | "yd" => Some((Measurement::YARD, String::from("Yard"))),
            "miles" | "mile" | "mi" => Some((Measurement::MILE, String::from("Mile"))),

            _ => None,
        }
    }
}
pub struct Measurement;

impl Measurement {
    pub const KILOMETER: f32 = 1000.0; // 1 km = 1000 meters
    pub const CENTIMETER: f32 = 0.01; // 1 cm = 0.01 meters
    pub const MILLIMETER: f32 = 0.001; // 1 mm = 0.001 meters
    pub const MICROMETER: f32 = 0.000_001; // 1 μm = 0.000001 meters

    pub const INCH: f32 = 0.0254; // 1 inch = 0.0254 meters
    pub const FEET: f32 = 0.3048; // 1 foot = 0.3048 meters
    pub const YARD: f32 = 0.9144; // 1 yard = 0.9144 meters
    pub const MILE: f32 = 1609.34; // 1 mile = 1609.34 meters
}
