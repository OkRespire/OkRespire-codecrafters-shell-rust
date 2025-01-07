#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let input = user_input();
        match input.as_str().trim() {
            "exit 0" => exit(0),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}

pub fn user_input() -> String {
    print!("$ ");
    io::stdout().flush().unwrap();
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    input
}

pub fn exit(code: i32) -> ! {
    std::process::exit(code)
}
