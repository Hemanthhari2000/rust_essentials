use std::io;
use std::str::FromStr;

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn main() {
    let mut input = String::new();
    println!("Please enter your weight on Earth (in Kgs)...");
    if let Ok(_) = io::stdin().read_line(&mut input) {
        input = input.trim().to_string();
        if let Ok(weight_on_earth) = f32::from_str(&input) {
            println!("Weight on Earth: {}Kg", weight_on_earth);
            let weight_on_mars = calculate_weight_on_mars(100.0);
            println!("Weight on Mars: {}Kg", weight_on_mars);
        } else {
            println!("Please enter a valid weight.");
        }
    } else {
        println!("Some error occured while reading the input.\nPlease try again.");
    }
}
