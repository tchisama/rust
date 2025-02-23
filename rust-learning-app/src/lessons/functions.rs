// Functions
//
pub fn main() {
    // Basic function
    fn greet() {
        println!("Hello, world!");
    }
    greet();

    // Function with parameters
    fn print_sum(a: i32, b: i32) {
        println!("Sum: {}", a + b);
    }
    print_sum(5, 10);

    // Function with return value
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let result = add(15, 20);
    println!("Result: {}", result);
}
