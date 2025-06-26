// src/lib.rs

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn square(n: i32) -> i32 {
    n * n
}
