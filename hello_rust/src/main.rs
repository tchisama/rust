use std::io;
use colored::*;

fn main() {
    println!("{}", "What's your name?".green());

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();
    println!("Hello, {}! Welcome to Rust.", name.blue().bold());
}
