#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let input = user_input();
        match input.as_str() {
            "exit" => break,
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
