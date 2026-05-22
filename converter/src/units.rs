#[derive(Debug, PartialEq)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl std::str::FromStr for TempUnit {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "c" => Ok(TempUnit::Celsius),
            "f" => Ok(TempUnit::Fahrenheit),
            "k" => Ok(TempUnit::Kelvin),
            _ => Err(format!("Unknown unit: '{}'", value)),
        }
    }
}

impl TempUnit {
    pub fn to_kelvin(&self, v: f64) -> f64 {
        match &self {
            TempUnit::Celsius => v + 273.15,
            TempUnit::Fahrenheit => (v + 459.67) * (5.0 / 9.0),
            TempUnit::Kelvin => v,
        }
    }

    pub fn from_kelvin(&self, v: f64) -> f64 {
        match &self {
            TempUnit::Celsius => v - 273.15,
            TempUnit::Fahrenheit => (v * (9.0 / 5.0)) - 459.67,
            TempUnit::Kelvin => v,
        }
    }
}

pub struct Temperature {
    pub value: f64,
    pub unit: TempUnit,
}

impl Temperature {
    pub fn convert_to(&self, target: TempUnit) -> Temperature {
        let value_kelvin = self.unit.to_kelvin(self.value);
        let value_target = target.from_kelvin(value_kelvin);

        return Temperature {
            value: value_target,
            unit: target,
        };
    }
}

#[cfg(test)]
mod temp_unit_from_string_tests {
    use super::*;

    #[test]
    fn test_celsius() {
        let t = "C".parse::<TempUnit>().unwrap();
        assert_eq!(t, TempUnit::Celsius);
    }

    #[test]
    fn test_fahrenheit() {
        let t = "F".parse::<TempUnit>().unwrap();
        assert_eq!(t, TempUnit::Fahrenheit);
    }

    #[test]
    fn test_kelvin() {
        let t = "K".parse::<TempUnit>().unwrap();
        assert_eq!(t, TempUnit::Kelvin);
    }

    #[test]
    fn test_unknown_unit() {
        let t = "A".parse::<TempUnit>();
        assert!(t.is_err());
    }
}

#[cfg(test)]
mod temp_unit_convert_tests {
    use super::*;
    use approx::assert_abs_diff_eq;

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

#[cfg(test)]
mod temperature_convert_tests {
    use super::*;

    #[test]
    fn test_convert_temperature() {
        let v = 10.0;
        let u1 = TempUnit::Celsius;
        let u2 = TempUnit::Fahrenheit;
        let u3 = TempUnit::Kelvin;

        let t1 = Temperature { value: v, unit: u1 };
        let t2 = t1.convert_to(u2);
        let t3 = t2.convert_to(u3);

        assert_ne!(t1.value, t2.value);
        assert_ne!(t2.value, t3.value);
        assert_ne!(t3.value, t1.value);
    }
}
