fn main() {
    let number = 3;

    // Using match as control flow
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 | 5 => println!("Four or Five"),      // multiple match using |
        6..=10 => println!("Between 6 and 10"), // range match
        _ => println!("Something else"),        // wildcard/default
    }
}
