pub fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("Now the value of y is: {}", y);

    // Shadowing
    let z = 5;
    let z = z + 1;
    println!("The value of z after shadowing is: {}", z);
}

// NOTE:
// 1. Rust is a statically typed language, which means that it must know the types of all variables
//    at compile time, however, the compiler can usually infer what type we want to use based on
//    the value and how we use it.
// 2. Rust is a block-scoped language, which means that the scope of a variable is limited to the
//   block in which it is declared.
// 3. Variables are immutable by default in Rust. We can make them mutable by using the `mut`
//    keyword.
// 4. Shadowing is a feature in Rust that allows us to declare a new variable with the same name as
//   a previous variable. This feature is useful when we want to change the type of a variable or
//   reassign a variable while keeping the original variable immutable.
