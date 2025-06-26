use std::collections::HashMap;

fn main() {
    // 1. Create empty HashMap
    let mut scores = HashMap::new();

    // 2. Insert key-value pairs
    scores.insert("Rust", 95);
    scores.insert("Python", 90);
    scores.insert("C", 80);

    // 3. Access value using .get()
    if let Some(score) = scores.get("Rust") {
        println!("Rust Score: {}", score);
    }

    // 4. Overwrite value
    scores.insert("C", 85); // C score updated from 80 to 85

    // 5. Check if key exists before inserting using .entry()
    scores.entry("Java").or_insert(88);
    scores.entry("Rust").or_insert(100); // won't overwrite, already exists

    // 6. Loop through all key-value pairs
    println!("\nAll Scores:");
    for (lang, score) in &scores {
        println!("{}: {}", lang, score);
    }

    // 7. Update value based on old one
    if let Some(val) = scores.get_mut("Python") {
        *val += 5;
    }

    println!("\nAfter updating Python:");
    for (lang, score) in &scores {
        println!("{}: {}", lang, score);
    }

    // 8. Ownership rules
    let name = String::from("Alice");
    let city = String::from("Delhi");
    let mut map = HashMap::new();
    map.insert(name, city); // name and city are moved here

    // println!("{}", name); // Error: value borrowed after move
    println!("\nCity map: {:?}", map);

    // 9. Remove key
    scores.remove("C");
    println!("\nAfter removing C: {:?}", scores);
}
