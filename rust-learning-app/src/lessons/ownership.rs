pub fn main() {
    println!("=== Ownership Basics ===");
    ownership_basics();

    println!("\n=== References and Borrowing ===");
    references_and_borrowing();

    println!("\n=== Slices ===");
    slices();
}

fn ownership_basics() {
    // Ownership rules:
    // 1. Each value in Rust has a variable that’s its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value is dropped.

    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1; // s1's ownership is moved to s2
    // println!("{}", s1); // This would cause a compile-time error because s1 no longer owns the String
    println!("s2: {}", s2);

    // To create a deep copy, use the `clone` method
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);

    // Ownership with functions
    let s4 = String::from("world");
    take_ownership(s4); // s4's ownership is moved into the function
    // println!("{}", s4); // This would cause a compile-time error because s4 no longer owns the String

    let s5 = give_ownership(); // Ownership is returned from the function
    println!("s5: {}", s5);
}

fn take_ownership(s: String) {
    println!("Inside take_ownership: {}", s);
} // s goes out of scope and is dropped

fn give_ownership() -> String {
    let s = String::from("hello");
    s // Ownership is returned
}

fn references_and_borrowing() {
    // Borrowing allows you to pass references to a value without transferring ownership
    let s1 = String::from("hello");

    // Immutable reference
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable reference
    let mut s2 = String::from("hello");
    change_string(&mut s2);
    println!("s2 after mutation: {}", s2);

    // Rules of references:
    // 1. You can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.
    let s3 = String::from("hello");

    let r1 = &s3; // Immutable reference
    let r2 = &s3; // Another immutable reference
    println!("r1: {}, r2: {}", r1, r2);

    // let r3 = &mut s3; // This would cause a compile-time error because immutable and mutable references cannot coexist
    // println!("{}, {}, {}", r1, r2, r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

fn slices() {
    // Slices let you reference a contiguous sequence of elements in a collection
    let s = String::from("hello world");

    let hello = &s[0..5]; // Slice from index 0 to 4
    let world = &s[6..11]; // Slice from index 6 to 10
    println!("hello: {}, world: {}", hello, world);

    // String literals are slices
    let s2 = "hello world";
    let hello2 = &s2[0..5];
    let world2 = &s2[6..11];
    println!("hello2: {}, world2: {}", hello2, world2);

    // Array slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // Slice from index 1 to 2
    println!("slice: {:?}", slice);
}
