// Normal struct
struct Person {
    name: String,
    age: u8,
    city: String,
}

// Tuple struct
struct Color(u8, u8, u8);

fn main() {
    // Using normal struct
    let person1 = Person {
        name: String::from("Raushan"),
        age: 21,
        city: String::from("Bihar"),
    };

    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("City: {}", person1.city);

    // Struct update syntax
    let person2 = Person {
        name: String::from("Aman"),
        ..person1
    };

    println!("New person: {} from {}", person2.name, person2.city);

    // ✅ Using tuple struct
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);

    // Access tuple struct values using .0, .1, .2
    println!("Red: {}, {}, {}", red.0, red.1, red.2);
    println!("Green: {}, {}, {}", green.0, green.1, green.2);
    println!("Blue: {}, {}, {}", blue.0, blue.1, blue.2);
}
