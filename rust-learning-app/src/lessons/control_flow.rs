pub fn main() {
    // If-else   NOTE: Rust requires that the condition must be a boolean
    let number = 7;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    // match
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
    // if-else with match
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        match number {
            1 => println!("One"),
            2 => println!("Two"),
            3 => println!("Three"),
            _ => println!("Something else"),
        }
    }
    // if else inside variables
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Loops  NOTE: Rust has three types of loops: loop, while, and for
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Skipping 3");
            continue;
        }
        println!("Count: {}", count);
        if count == 5 {
            println!("Breaking at 5");
            break;
        }
    }

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // For loop
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("The value is: {}", element);
    }
}
