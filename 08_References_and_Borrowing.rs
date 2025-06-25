fn main() {
    let name = String::from("Rustacean");

    // Immutable reference
    print_name(&name);        // borrowing without taking ownership
    println!("Main still owns: {}", name); // still valid

    // Mutable reference
    let mut language = String::from("Rust");
    change_language(&mut language);
    println!("Changed to: {}", language);

    // Scope of references
    let mut s = String::from("Hello");

    {
        let r1 = &s;           // immutable borrow
        println!("Inside scope: {}", r1); // r1 ends here
    }

    let r2 = &mut s;          // mutable borrow allowed after immutable ends
    r2.push_str(" World");
    println!("After push: {}", r2);

    // ❌ Invalid: can't have both mutable and immutable at same time
    /*
    let r3 = &s;
    let r4 = &mut s;
    println!("{} {}", r3, r4); // Error
    */

    // ❌ Dangling reference not allowed
    /*
    let ref_str = dangling(); // Error
    println!("{}", ref_str);
    */
}

// Immutable reference
fn print_name(n: &String) {
    println!("Name is: {}", n);
}

// Mutable reference
fn change_language(lang: &mut String) {
    lang.push_str(" Language");
}

// ❌ This would cause a dangling reference
/*
fn dangling() -> &String {
    let s = String::from("Oops");
    &s   // returns reference to local variable (will be dropped)
}
*/
