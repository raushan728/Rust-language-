fn main() {
    let mut numbers = vec![100, 200, 300];

    numbers.push(400); // pushing because it's mutable

    // Loop
    for num in &numbers {
        println!("Value: {}", num);
    }

    // Index access
    println!("Index 2 ([]) = {}", numbers[2]);

    // Safe access with get
    match numbers.get(10) {
        Some(val) => println!("Index 10 = {}", val),
        None => println!("Index 10 out of bounds"),
    }

    // Ownership scope
    {
        numbers.push(500);
        println!("Inside block: {:?}", numbers);
    }

    println!("Final Vector: {:?}", numbers);
}
