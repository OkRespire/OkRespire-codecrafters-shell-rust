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
            "echo" => {
                println!("{}", args.join(" "))
            }
            "type" => {
                if builtins.contains(&args[0].as_str()) {
                    println!("{} is a shell builtin", args[0])
                } else {
                    //used nicklasmoeller solution

                    let split_paths = &mut path_var.split(":");
                    //checks if the argument is in the path.
                    //i.e. cat is located in /usr/bin
                    if let Some(path) = split_paths
                        .find(|path| std::fs::metadata(format!("{}/{}", path, args[0])).is_ok())
                    {
                        println!("{} is {}", args[0], path.to_owned() + "/" + &args[0]);
                    } else {
                        println!("{}: not found", args[0]);
                    }
                }
            }
            "pwd" => {
                let curr_dir = std::env::current_dir().unwrap().display().to_string();
                println!("{}", curr_dir);
            }
            "cd" => {
                change_directory(&args[0]);
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

pub fn user_input() -> (String, Vec<String>) {
    print!("$ ");
    io::stdout().flush().unwrap();
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let input = input.trim();

    //looks at the first bit of the input and then the arguments
    let command = input
        .split_whitespace() //converts the input into a vector
        .next() //gets the first value
        .unwrap() //checks if there is a value in the input
        .to_string(); //converts the first value to a string

    //makes arguments a vector instead of a string
    let mut args = Vec::new();
    let mut current = String::new(); //looks at the current argument
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    //looks at each character in the input
    for char in input.chars() {
        match char {
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes; //single quotes are either opened or closed,
                                                      //depending on the state
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes; //double quotes are either opened or closed,
                                                      //depending on the state
            }
            ' ' if !in_single_quotes && !in_double_quotes => {
                //if space outside quote
                if !current.is_empty() {
                    args.push(current.clone()); //add current arg to vector
                    current.clear(); //resets the current string
                }
            }
            _ => current.push(char), //if it's not a space, add it to the current
        }
    }

    //adds last argument if there is any
    if !current.is_empty() {
        args.push(current.clone());
    }

    let command = args.first().cloned().unwrap(); //the first word is the command
    let arguments = args.into_iter().skip(1).collect(); //collects all the arguments
                                                        //minus the first one as it is a command
    (command, arguments)
}

pub fn execute_command(command: &str, _args: &[String]) {
    let mut cmd = std::process::Command::new(command)
        .args(_args)
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
