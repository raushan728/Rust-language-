// src/main.rs

// Bring library functions into scope
use rust_package_demo::{greet, square};

fn main() {
    let name = "Raushan";
    let message = greet(name);
    println!("{}", message);

    let result = square(6);
    println!("Square is: {}", result);
}
