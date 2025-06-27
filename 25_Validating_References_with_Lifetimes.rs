use std::fmt;

// ---------- TRAIT ----------
trait Summary {
    fn summary(&self) -> String;
}

// ---------- STRUCT ----------
struct User<'a> {
    username: String,
    email: String,
    age: u8,
    bio: &'a str, // lifetime reference
}

// ---------- IMPLEMENT TRAIT ----------
impl<'a> Summary for User<'a> {
    fn summary(&self) -> String {
        format!("{} ({}) - {}", self.username, self.age, self.bio)
    }
}

// ---------- FUNCTIONS ----------

// Function that takes ownership
fn print_user(user: User) {
    println!("[print_user] {}", user.summary());
    // user is dropped after this
}

// Function that borrows (no ownership move)
fn print_name(name: &String) {
    println!("[print_name] Name: {}", name);
}

// Function returning reference (needs lifetime)
fn longest_bio<'a>(b1: &'a str, b2: &'a str) -> &'a str {
    if b1.len() > b2.len() {
        b1
    } else {
        b2
    }
}

fn main() {
    // Create User
    let bio1 = "Love Rust and building tools.";
    let bio2 = "Enjoys systems programming.";

    let user1 = User {
        username: String::from("Raushan"),
        email: String::from("raushan@example.com"),
        age: 21,
        bio: bio1,
    };

    // Print basic info
    println!("User Summary: {}", user1.summary());

    // Borrow username (no move)
    print_name(&user1.username);

    // Compare bios
    let longest = longest_bio(bio1, bio2);
    println!("Longest bio: {}", longest);

    // Clone user1 to pass ownership
    let user2 = User {
        username: user1.username.clone(),
        email: user1.email.clone(),
        age: user1.age,
        bio: user1.bio,
    };

    print_user(user2); // moves ownership
    // user2 can't be used after this

    println!("Program completed.");
}
