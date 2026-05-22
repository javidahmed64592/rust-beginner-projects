use std::env;
mod units;
use crate::units::TempUnit;
use crate::units::Temperature;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_args = args.len();
    let v = args[num_args - 3].parse::<f64>().unwrap();
    let u1: TempUnit = args[num_args - 2].parse().expect("Invalid unit");
    let u2: TempUnit = args[num_args - 1].parse().expect("Invalid unit");

    let temp = Temperature { value: v, unit: u1 };
    let result = temp.convert_to(u2);
    println!("{}", result.value);
}
