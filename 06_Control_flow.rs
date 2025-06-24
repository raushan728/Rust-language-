fn main() {
    // If-else condition
    let number = 10;

    if number < 0 {
        println!("Negative number");
    } else if number == 0 {
        println!("Zero");
    } else {
        println!("Positive number");
    }

    // loop with break
    let mut count = 0;
    loop {
        if count == 3 {
            break;
        }
        println!("loop count: {}", count);
        count += 1;
    }

    // while loop
    let mut num = 5;
    while num > 0 {
        println!("while loop: {}", num);
        num -= 1;
    }

    // for loop with continue and break
    for i in 1..=5 {
        if i == 3 {
            continue; // skip 3
        }
        if i == 5 {
            break; // stop at 5
        }
        println!("for loop: {}", i);
    }
}
