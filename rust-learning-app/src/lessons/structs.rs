/*
NOTE: simple example of struct

 I see you're interested in learning about Rust structs. In Rust, a struct is a custom data type that lets you group named fields of possibly different types together. Here's a simple example to help you understand:
```rust
// Define a struct named `Person`
struct Person {
    name: String,
    age: u8,
}
fn main() {
    // Create an instance of the `Person` struct
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    // Access the fields of the struct
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
}
```
In this example, we define a `Person` struct with `name` and `age` fields. We then create an instance of the `Person` struct and access its fields using dot notation.
Structs in Rust can also have methods associated with them using `impl` blocks. This allows you to define behavior specific to instances of the struct.
I hope this helps you get started with Rust structs! Let me know if you have any specific questions or if you'd like to see more examples.

*/

pub fn main() {
    println!("=== Defining and Instantiating Structs ===");
    defining_structs();

    println!("\n=== Tuple Structs ===");
    tuple_structs();

    println!("\n=== Unit-Like Structs ===");
    unit_like_structs();

    println!("\n=== Methods and Associated Functions ===");
    methods_and_associated_functions();

    println!("\n=== Derived Traits for Structs ===");
    derived_traits();
}

// === Defining and Instantiating Structs ===
fn defining_structs() {
    // Define a struct
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // Instantiate the struct
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };

    println!("User1: {:?}", user1);

    // Create a mutable instance
    let mut user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        active: false,
        sign_in_count: 0,
    };

    // Modify a field
    user2.sign_in_count = 1;
    user2.active = true;
    user2.email = String::from("tchisama@contact.me");
    user2.username = String::from("tchisama");
    // println!("User2: {:?}", user2);

    // Struct update syntax
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user1 // Use the rest of the fields from user1
    };
    println!("User3: {:?}", user3);
}

// === Tuple Structs ===
fn tuple_structs() {
    // Define a tuple struct
    #[derive(Debug)]
    struct Color(i32, i32, i32);

    // Instantiate the tuple struct
    let black = Color(0, 0, 0);
    println!("Black: {:?}", black);

    // Access tuple struct fields
    println!("Red component: {}", black.0);
}

// === Unit-Like Structs ===
fn unit_like_structs() {
    // Define a unit-like struct
    #[derive(Debug)]
    struct AlwaysEqual;

    // Instantiate the unit-like struct
    let subject = AlwaysEqual;
    println!("AlwaysEqual: {:?}", subject);
}

// === Methods and Associated Functions ===
fn methods_and_associated_functions() {
    // Define a struct with methods
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // Implement methods for the struct
    impl Rectangle {
        // Method to calculate area
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Method to check if one rectangle can hold another
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // Associated function (doesn't take `self` as a parameter)
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    // Instantiate the struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Call methods
    println!("Area of rect1: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Call an associated function
    let square = Rectangle::square(20);
    println!("Square: {:?}", square);
}

// === Derived Traits for Structs ===
fn derived_traits() {
    // Derive common traits for a struct
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone(); // Clone the struct
    let p3 = p1; // Copy the struct (because it implements Copy)

    println!("p1: {:?}, p2: {:?}, p3: {:?}", p1, p2, p3);
    println!("Are p1 and p2 equal? {}", p1 == p2);
}
