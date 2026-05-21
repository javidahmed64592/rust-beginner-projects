use approx::assert_abs_diff_eq;

enum TempUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TempUnit {
    fn to_kelvin(&self, v: f64) -> f64 {
        match &self {
            TempUnit::Celsius => v + 273.15,
            TempUnit::Fahrenheit => (v + 459.67) * (5.0 / 9.0),
            TempUnit::Kelvin => v,
        }
    }

    fn from_kelvin(&self, v: f64) -> f64 {
        match &self {
            TempUnit::Celsius => v - 273.15,
            TempUnit::Fahrenheit => (v * (9.0 / 5.0)) - 459.67,
            TempUnit::Kelvin => v,
        }
    }
}

#[cfg(test)]
mod convert_tests {
    use super::*;

    #[test]
    fn test_celsius_to_kelvin() {
        let t = TempUnit::Celsius;
        assert_eq!(t.to_kelvin(0.0), 273.15);
    }

    #[test]
    fn test_celsius_from_kelvin() {
        let t = TempUnit::Celsius;
        assert_eq!(t.from_kelvin(0.0), -273.15);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        let t = TempUnit::Fahrenheit;
        assert_abs_diff_eq!(t.to_kelvin(0.0), 255.3722, epsilon = 0.01);
    }

    #[test]
    fn test_fahrenheit_from_kelvin() {
        let t = TempUnit::Fahrenheit;
        assert_abs_diff_eq!(t.from_kelvin(258.15), 5.0, epsilon = 0.01);
    }

    #[test]
    fn test_kelvin_to_kelvin() {
        let t = TempUnit::Kelvin;
        assert_eq!(t.to_kelvin(10.0), 10.0);
    }

    #[test]
    fn test_kelvin_from_kelvin() {
        let t = TempUnit::Kelvin;
        assert_eq!(t.from_kelvin(10.0), 10.0);
    }
}
