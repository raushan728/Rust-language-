// Step 1: Basic enum
enum Direction {
    North,
    South,
    East,
    West,
}

// Step 2: Enum with data
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
}

fn main() {
    // Using simple enum
    let dir = Direction::East;

    match dir {
        Direction::North => println!("Moving North"),
        Direction::South => println!("Moving South"),
        Direction::East => println!("Moving East"),
        Direction::West => println!("Moving West"),
    }

    // Using enum with data
    let msg1 = Message::Write(String::from("Hello Rust"));
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::ChangeColor(255, 0, 0);

    show_message(msg1);
    show_message(msg2);
    show_message(msg3);

    // Using built-in Option enum
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("Got number: {}", n),
        None => println!("No number found"),
    }

    match no_number {
        Some(n) => println!("Got: {}", n),
        None => println!("Nothing here"),
    }

    // match with wildcard (_)
    let coin = "Unknown";

    match coin {
        "Heads" => println!("It's Heads!"),
        "Tails" => println!("It's Tails!"),
        _ => println!("Not a valid coin"), // wildcard matches anything else
    }
}

// Function to match Message enum
fn show_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received"),
        Message::Write(text) => println!("Message: {}", text),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}
