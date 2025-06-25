// Step 1: Define a struct
struct Student {
    name: String,
    age: u8,
}

// Step 2: Implement methods using 'impl' block
impl Student {
    // Method to display details
    fn show(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }

    // Method to check if student is adult
    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    // Method that takes mutable self
    fn birthday(&mut self) {
        self.age += 1;
    }
}
    
fn main() {
    // Create an instance
    let mut s1 = Student {
        name: String::from("Raushan"),
        age: 19,
    };

    // Call methods
    s1.show(); // Name: Raushan, Age: 17
    println!("Adult? {}", s1.is_adult()); // false

    s1.birthday(); // age +1
    s1.show(); // Name: Raushan, Age: 18
    println!("Adult? {}", s1.is_adult()); // true
}
