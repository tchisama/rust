// get variabales in ./lessons/variables.rs

mod lessons {
    pub mod control_flow;
    pub mod data_types;
    pub mod functions;
    pub mod ownership;
    pub mod structs;
    pub mod variables;
}

fn main() {
    let lesson = "Structs";

    match lesson {
        "Data Types" => lessons::data_types::main(),
        "Variables" => lessons::variables::main(),
        "Control Flow" => lessons::control_flow::main(),
        "Functions" => lessons::functions::main(),
        "Ownership" => lessons::ownership::main(),
        "Structs" => lessons::structs::main(),
        _ => println!("No lesson found"),
    }
}

// NOTE:
// 1. use `mod` to import modules
// 2. use `pub` to make modules public
// 3. use `::` to access modules
