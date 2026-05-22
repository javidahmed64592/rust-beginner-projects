use std::env;
mod units;
use crate::units::TempUnit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_args = args.len();
    let v = &args[num_args - 3].parse::<f64>().unwrap();
    let u1: &TempUnit = &args[num_args - 2].parse().expect("Invalid unit");
    let u2: &TempUnit = &args[num_args - 1].parse().expect("Invalid unit");

    let v_to_kelvin = u1.to_kelvin(*v);
    let v_to_u2 = u2.from_kelvin(v_to_kelvin);
    println!("{}", v_to_u2);
}
