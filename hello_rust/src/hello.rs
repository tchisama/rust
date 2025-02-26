



fn hello_rust() {
    println!("{}", "What's your name?".green());

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();
    println!("hiiii", name.blue().bold());
}

fn main() {
    hello_rust();
}






