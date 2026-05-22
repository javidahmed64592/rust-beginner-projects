use std::env;

fn calculate(a: f64, o: &str, b: f64) -> Result<f64, &str> {
    match o {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("Division by zero!")
            } else {
                Ok(a / b)
            }
        }
        &_ => Err("Invalid operator!"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: calculator <number> <operator> <number>");
        std::process::exit(1);
    }

    let a = match args[1].parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid value: {} (expected a number)", args[1]);
            std::process::exit(1);
        }
    };
    let o = &args[2];
    let b = match args[3].parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid value: {} (expected a number)", args[3]);
            std::process::exit(1);
        }
    };

    let result = calculate(a, o, b);

    println!("{}", result.unwrap());
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
