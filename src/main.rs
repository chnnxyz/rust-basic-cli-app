// calculate weight for different gravities

use std::io;

const EARTH_GRAVITY: f32 = 9.81;

fn main() {
    // declare vars
    let mut input = String::new();
    let mut gravity = String::new();

    // Read io
    println!("Provide weight in kg:");
    io::stdin().read_line(&mut input).unwrap();

    println!("Provide target planet gracity:");
    io::stdin().read_line(&mut gravity).unwrap();

    // Convert io
    let input = input.trim();
    let gravity = gravity.trim();
    let input = input.parse::<f32>().unwrap();
    let gravity = gravity.parse::<f32>().unwrap();

    // release memory and get output
    let result = weight(input, gravity);
    // print out
    println!("Weight on other planet is {}", result);
}

fn weight(weight: f32, gravity:f32) -> f32 {
    weight * gravity / EARTH_GRAVITY
}
