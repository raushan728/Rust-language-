fn main() {
    // 1. Create String
    let s1 = String::new();
    let s2 = String::from("Hello");
    let s3 = "World".to_string();
    println!("s1 = '{}', s2 = '{}', s3 = '{}'", s1, s2, s3);

    // 2. UTF-8 text
    let greet = String::from("नमस्ते 🌍");
    println!("UTF-8 Greeting: {}", greet);

    // 3. push() and push_str()
    let mut name = String::from("Raushan");
    name.push(' ');
    name.push_str("Singh");
    println!("Full Name: {}", name);

    // 4. + operator (moves ownership)
    let s4 = String::from("Hello");
    let s5 = String::from(" Rust");
    let joined = s4 + &s5;
    println!("Joined with +: {}", joined);

    // 5. format! macro (best way to combine)
    let f_name = "Raushan";
    let course = "Rust";
    let msg = format!("{} is learning {}", f_name, course);
    println!("{}", msg);

    // 6. len() (byte length)
    let hindi = String::from("नमस्ते");
    println!("'नमस्ते' Length: {} bytes", hindi.len());

    // 7. chars() - iterate over characters
    print!("Characters: ");
    for c in hindi.chars() {
        print!("{} ", c);
    }
    println!();

    // 8. bytes() - raw bytes
    print!("Bytes: ");
    for b in hindi.bytes() {
        print!("{} ", b);
    }
    println!();

    // 9. get() with match
    match hindi.get(0..6) {
        Some(slice) => println!("Safe Slice: {}", slice),
        None => println!("Invalid slice"),
    }

    // 10. Useful methods: contains, replace, trim, to_uppercase
    let raw = String::from(" hello rust ");
    println!("Trimmed: '{}'", raw.trim());
    println!("Contains 'rust'? {}", raw.contains("rust"));
    println!("Replace: {}", raw.replace("rust", "world"));
    println!("Uppercase: {}", raw.to_uppercase());

    // 11. Final Combined Example
    let mut final_msg = String::new();
    final_msg.push_str("Welcome ");
    final_msg.push_str("to Rust!");
    println!("{}", final_msg);
}
