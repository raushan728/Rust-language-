fn main() {
    // Default types
    let int_num = 42;             // i32 by default
    let float_num = 3.1415;       // f64 by default
    let is_valid = false;         // bool
    let letter = 'R';             // char
    let name = "Raushan";         // &str (string slice)

    // Explicit types
    let small_num: u8 = 255;
    let big_float: f32 = 5.5;

    // String (owned, growable)
    let mut city = String::from("Patna");
    city.push_str(" Bihar");

    println!("int_num = {}", int_num);
    println!("float_num = {}", float_num);
    println!("is_valid = {}", is_valid);
    println!("letter = {}", letter);
    println!("name = {}", name);
    println!("small_num = {}", small_num);
    println!("big_float = {}", big_float);
    println!("city = {}", city);
    // Array example
    let numbers = [1, 2, 3, 4, 5];
    println!("First element of array: {}", numbers[0]);
    println!("Array length: {}", numbers.len());

    // Tuple example
    let person: (&str, i32, f64) = ("Alice", 30, 5.7);
    println!("Name: {}, Age: {}, Height: {}", person.0, person.1, person.2);
}
