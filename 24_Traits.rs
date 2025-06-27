// 1. Define a trait
trait Greet {
    fn greet(&self); // required method

    fn say_bye(&self) {
        println!("Goodbye!"); // default method
    }
}

// 2. Create a struct
struct Student {
    name: String,
    age: u8,
}

struct Teacher {
    subject: String,
    name: String,
}

// 3. Implement trait for Student
impl Greet for Student {
    fn greet(&self) {
        println!("Hi, I'm student {} and I'm {} years old.", self.name, self.age);
    }

    // use default say_bye()
}

// 4. Implement trait for Teacher
impl Greet for Teacher {
    fn greet(&self) {
        println!("Hello, I'm teacher {} and I teach {}.", self.name, self.subject);
    }

    fn say_bye(&self) {
        println!("Teacher {} says: See you later!", self.name);
    }
}

// 5. Use trait object in a function
fn welcome(person: &impl Greet) {
    person.greet();
    person.say_bye();
}
// 6. Main function
fn main() {
    let s1 = Student {
        name: String::from("Raushan"),
        age: 20,
    };

    let t1 = Teacher {
        name: String::from("Mr. Verma"),
        subject: String::from("Rust Programming"),
    };

    welcome(&s1);
    println!("---");
    welcome(&t1);
}
