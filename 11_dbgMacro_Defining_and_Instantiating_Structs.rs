#[derive(Debug)]  // ← Required to enable printing with {:?} or dbg!
struct Student {
    name: String,
    age: u8,
    grade: char,
}

fn main() {
    let student = Student {
        name: String::from("Raushan"),
        age: 21,
        grade: 'A',
    };

    // 🔸 Use dbg! macro to debug-print the struct
    dbg!(&student);

    // 🔸 You can also assign the output of dbg!
    let age = dbg!(student.age + 1);
    println!("Next year age: {}", age);
}
