fn main() {
    // 1. Manual panic
    // panic!("Manual panic occurred!");

    // 2. Index out of bounds
    let data = vec![1, 2, 3];
    // println!("Accessing invalid index: {}", data[10]); // panic

    // 3. Safe access using get()
    match data.get(10) {
        Some(val) => println!("Safe access: {}", val),
        None => println!("Handled out of bounds safely with get()"),
    }

    // 4. unwrap and expect
    let ok_val: Option<&str> = Some("Rust");
    let bad_val: Option<&str> = None;

    println!("Unwrap ok_val: {}", ok_val.unwrap()); // works

    // println!("Unwrap bad_val: {}", bad_val.unwrap()); // panic
    // println!("Expect bad_val: {}", bad_val.expect("Value was None")); // panic with custom msg

    println!("All done safely.");
}
