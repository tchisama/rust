pub fn main() {
    // Scalar Types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';
    let _unit: () = ();
    let _pointer: *const i32 = &integer;

    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("Unit: {:?}", _unit);
    println!("Pointer: {:?}", _pointer);

    // Compound Types
    let tuple: (i32, f64, char) = (42, 3.14, 'R');
    println!("Tuple: {:?}", tuple);

    let array: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", array);
}

// NOTE:
// 1. Rust has two categories of data types: scalar and compound.
// 2. diff between tuple and array is that tuple can have elements of different types, while array
//    can have elements of the same type.
// 3. Rust has a special type called `unit` that has only one value, which is also written as `()`.
// 4. Rust has a special type called `pointer` that stores the memory address of a value. We can
//
// HACK:
// 1. Try changing the type of the variables and see how the compiler reacts.
//
