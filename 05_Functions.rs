fn main() {
    greet();                       // function with no parameters and no return value

    let sum = add(5, 7);           // function with parameters and return value
    println!("Sum is: {}", sum);

    let squared = square(4);       // expression as return
    println!("Square is: {}", squared);

    let nothing = do_nothing();    // function returns unit type ()
    println!("Nothing returns: {:?}", nothing);
}

fn greet() {
    println!("Hello, Rust!");      // statement
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;                  // return value using 'return' keyword
}

fn square(x: i32) -> i32 {
    x * x                          // return as expression (no semicolon)
}

fn do_nothing() {
    // no return value, returns ()
}
