fn main() {
    // Constant declaration (must have type and be uppercase by convention)
    const MAX_LIMIT: u32 = 100;
    println!("The constant MAX_LIMIT is: {}", MAX_LIMIT);

    // Variable declaration using 'let'
    let x = 10;
    println!("Initial value of x: {}", x);

    // Shadowing x with a new value (can change type as well)
    let x = x + 5;
    println!("After shadowing x (x + 5): {}", x);

    // Shadowing x again with a different type (from integer to string)
    let x = "Now I am a string!";
    println!("After shadowing x again with string: {}", x);

    // Another variable using 'let'
    let y = 3.14;
    println!("Value of y: {}", y);

    // Mutable variable (not shadowing, just modifying)
    let mut z = 20;
    println!("Initial value of mutable z: {}", z);
    z = z + 1;
    println!("Modified value of z: {}", z);
}
