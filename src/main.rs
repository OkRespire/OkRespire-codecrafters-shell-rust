#[allow(unused_imports)]
use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    let path_var = std::env::var("PATH").unwrap();
    let builtins = HashSet::from(["exit", "echo", "type", "pwd", "cd"]);
    loop {
        let (command, args) = user_input();
        match command.as_str() {
            "exit" => exit(0),
            "echo" => println!("{}", args),
            "type" => {
                if builtins.contains(&args.as_str()) {
                    println!("{} is a shell builtin", args)
                } else {
                    //used nicklasmoeller solution

                    let split_paths = &mut path_var.split(":");
                    //checks if the argument is in the path.
                    //i.e. cat is located in /usr/bin
                    if let Some(path) = split_paths
                        .find(|path| std::fs::metadata(format!("{}/{}", path, args)).is_ok())
                    {
                        println!("{} is {}", args, path.to_owned() + "/" + &args.to_string());
                    } else {
                        println!("{}: not found", args);
                    }
                }
            }
            "pwd" => {
                let curr_dir = std::env::current_dir().unwrap().display().to_string();
                println!("{}", curr_dir);
            }
            "cd" => {
                change_directory(&args);
            }
            _ => {
                let split_paths = &mut path_var.split(":");
                if let Some(path) = split_paths
                    .find(|path| std::fs::metadata(format!("{}/{}", path, command)).is_ok())
                {
                    execute_command(&command, &args);
                } else {
                    println!("{}: command not found", command.trim());
                }
            }
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

pub fn execute_command(command: &str, _args: &str) {
    let mut cmd = std::process::Command::new(command)
        .args(_args.split_whitespace())
        .spawn()
        .expect("failed to execute process");

    cmd.wait().expect("failed to wait on child");
}

pub fn change_directory(directory: &str) {
    //if the start of the directory is ~, then it redirects to the home directory
    if directory.starts_with('~') {
        let home = std::env::var("HOME").unwrap();
        let new_dir = std::env::set_current_dir(home + &directory[1..]);
        if new_dir.is_err() {
            println!("cd: {}: No such file or directory", directory);
        }
        return;
    }

    let new_dir = std::env::set_current_dir(directory);

    if new_dir.is_err() {
        println!("cd: {}: No such file or directory", directory);
    }
}

pub fn exit(code: i32) -> ! {
    std::process::exit(code)
}
