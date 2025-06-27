use std::fs::File;
use std::io::{self, Read};

fn main() {
    // 1. Try opening a file (may or may not exist)
    let result = File::open("my_file.txt");

    // 2. Pattern matching
    let mut file = match result {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open file: {}", error);
            return;
        }
    };

    // 3. Read file content
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => println!("File content:\n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    // 4. Using unwrap and expect (unsafe, but sometimes ok)
    // let f = File::open("another.txt").unwrap(); // will panic if file missing
    // let f2 = File::open("another.txt").expect("Can't open file"); // panic with message
    

    // 5. Using a function that returns Result with ?
    match read_file("my_file.txt") {
        Ok(content) => println!("\n[Using ?] File says:\n{}", content),
        Err(e) => println!("Error using ?: {}", e),
    }
}

// 6. Function that returns Result
fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // if error, will return early
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
