fn main() {
    let s = String::from("Hello Rust!");

    // ✅ Creating a string slice (part of the original string)
    let hello = &s[0..5];   // from index 0 to 4
    let rust = &s[6..10];   // from index 6 to 9

    println!("Slice1: {}", hello);
    println!("Slice2: {}", rust);

    // ✅ Using full slice
    let full = &s[..];      // entire string slice
    println!("Full slice: {}", full);

    // ❌ Invalid: Cannot clear String while slice is active
    // s.clear();            // Error: cannot borrow `s` as mutable because it’s already borrowed
    println!("Original still: {}", s);

    // ✅ Array slice
    let arr = [1, 2, 3, 4, 5];
    let part = &arr[1..4];  // index 1 to 3
    println!("Array slice: {:?}", part);
}
