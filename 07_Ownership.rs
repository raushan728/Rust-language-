fn main() {
    // Integer: Copy type (stack)
    let x = 10;
    let y = x; // x is copied, not moved
    println!("x = {}, y = {}", x, y);

    // String: Heap allocated (ownership moves)
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
                 // println!("{}", s1);  // ❌ Error: s1 no longer owns the value
    println!("s2 = {}", s2);

    // Clone: make a deep copy
    let s3 = String::from("world");
    let s4 = s3.clone(); // both s3 and s4 are valid
    println!("s3 = {}, s4 = {}", s3, s4);

    // Passing ownership to function
    let name = String::from("Rust");
    takes_ownership(name);
    // println!("{}", name); // ❌ Error: name is moved into function

    // Returning ownership
    let new_name = gives_ownership();
    println!("new_name = {}", new_name);

    // Ownership returned from function
    let again = String::from("code");
    let again = takes_and_returns(again);
    println!("again = {}", again);
}

fn takes_ownership(s: String) {
    println!("Got ownership of: {}", s);
}

fn gives_ownership() -> String {
    let s = String::from("Returned");
    s // returned to caller
}

fn takes_and_returns(s: String) -> String {
    s // ownership returned
}
