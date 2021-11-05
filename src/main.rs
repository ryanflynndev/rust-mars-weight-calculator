use std::io;

fn main() {
    println!("Enter your weight (kg):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight: f32 = calculate(weight);

    println!("Weight on mars {}", mars_weight);
}

fn calculate(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}