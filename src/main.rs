#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let (command, args) = user_input();
        match command.as_str() {
            "exit" => exit(0),
            "echo" => println!("{}", args),
            _ => println!("{}: command not found", command.trim()),
        }
    }
}

pub fn user_input() -> (String, String) {
    print!("$ ");
    io::stdout().flush().unwrap();
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    //looks at the first bit of the input and then the arguments
    let command = input
        .split_whitespace() //converts the input into a vector
        .next() //gets the first value
        .unwrap() //checks if there is a value in the input
        .to_string(); //converts the first value to a string
    let args = input
        .split_whitespace() //splits the input to a vector
        .skip(1) //skips the first word of the vector
        .collect::<Vec<_>>() //collects next two values into a vector
        .join(" "); //joins the vector into a string

    (command, args)
}

pub fn exit(code: i32) -> ! {
    std::process::exit(code)
}
