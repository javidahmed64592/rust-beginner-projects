use std::env;

fn calculate(a: f64, o: &str, b: f64) -> Result<f64, String> {
    match o {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("Division by zero!".to_string())
            } else {
                Ok(a / b)
            }
        }
        &_ => Err("Invalid operator!".to_string()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_args = args.len();
    let a = &args[num_args - 3].parse().unwrap();
    let o = &args[num_args - 2];
    let b = &args[num_args - 1].parse().unwrap();

    let result = calculate(*a, o, *b);

    println!("{}", result.unwrap().to_string());
}

#[cfg(test)]
mod calculate_tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(2.0, "+", 3.0).unwrap(), 5.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(5.0, "-", 2.0).unwrap(), 3.0)
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(2.0, "*", 3.0).unwrap(), 6.0)
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(10.0, "/", 2.0).unwrap(), 5.0)
    }

    #[test]
    fn test_division_by_zero() {
        assert!(calculate(5.0, "/", 0.0).is_err());
    }
}
