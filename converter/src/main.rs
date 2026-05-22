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
    let v = args[1].parse::<f64>().unwrap();
    let u1: TempUnit = args[2].parse().expect("Invalid unit");
    let u2: TempUnit = args[3].parse().expect("Invalid unit");

    let temp = Temperature { value: v, unit: u1 };
    let result = temp.convert_to(u2);
    println!("{}", result.value);
}
