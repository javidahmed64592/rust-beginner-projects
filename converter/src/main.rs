use std::env;
mod units;
use crate::units::TempUnit;
use crate::units::Temperature;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: converter <value> <from_unit> <to_unit>");
        std::process::exit(1);
    }

    let v = match args[1].parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid value: {} (expected a number)", args[1]);
            std::process::exit(1);
        }
    };

    let u1: TempUnit = match args[2].parse() {
        Ok(u1) => u1,
        Err(_) => {
            eprintln!("Invalid unit: {} (expected C, F or K)", args[2]);
            std::process::exit(1);
        }
    };

    let u2: TempUnit = match args[3].parse() {
        Ok(u2) => u2,
        Err(_) => {
            eprintln!("Invalid unit: {} (expected C, F or K)", args[3]);
            std::process::exit(1);
        }
    };

    let temp = Temperature { value: v, unit: u1 };
    let result = temp.convert_to(u2);
    println!("{}", result.value);
}
