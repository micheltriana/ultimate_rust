use std::io::{stdin};

fn main() {

    println!("Enter your weigth on Earth (in kg):");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let earth_weight = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(earth_weight);
    println!("If your weight on Earth is {}kg, then on Mars it'd be {}kg", earth_weight, mars_weight);
}

fn calculate_weight_on_mars(earth_weight:f32) -> f32 {
    earth_weight / 9.81 * 3.711
}
