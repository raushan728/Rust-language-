fn main() {
    let name = Some("Raushan");

    if let Some(n) = name {
        println!("Hello, {}!", n);
    } else {
        println!("No name found");
    }
}
