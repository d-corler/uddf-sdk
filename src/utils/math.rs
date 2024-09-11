pub fn convert_celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

pub fn convert_bar_to_pascal(bar: f64, decimal_places: u32) -> f64 {
    let factor = 10f64.powi(decimal_places as i32);
    (bar * 100_000.0 * factor).round() / factor
}

pub fn semicircles_to_degrees(semicircles: i32, decimal_places: u32) -> f64 {
    let factor = 10f64.powi(decimal_places as i32);
    (semicircles as f64 * (180f64 / 2f64.powi(31)) * factor).round() / factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_celsius_to_kelvin() {
        assert_eq!(convert_celsius_to_kelvin(0.0), 273.15);
        assert_eq!(convert_celsius_to_kelvin(100.0), 373.15);
        assert_eq!(convert_celsius_to_kelvin(-273.15), 0.0);
        assert_eq!(convert_celsius_to_kelvin(25.0), 298.15);
    }

    #[test]
    fn test_convert_bar_to_pascal() {
        assert_eq!(convert_bar_to_pascal(1.0, 0), 100_000.0);
        assert_eq!(convert_bar_to_pascal(0.0, 0), 0.0);
        assert_eq!(convert_bar_to_pascal(2.0, 0), 200_000.0);
        assert_eq!(convert_bar_to_pascal(0.5, 0), 50_000.0);
        assert_eq!(convert_bar_to_pascal(78.57, 2), 7_857_000.0);
    }

    #[test]
    fn test_semicircles_to_degrees() {
        assert_eq!(semicircles_to_degrees(0, 1), 0.0);
        assert_eq!(semicircles_to_degrees(579933426, 10), 48.6094582267);
        assert_eq!(semicircles_to_degrees(-55384979, 10), -4.6423153114);
    }
}
