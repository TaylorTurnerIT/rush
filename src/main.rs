#[allow(unused_imports)]
use std::io::{self, Read, Write};

fn check_type(command: &str, builtin: [&str; 3]) {
    if builtin.contains(&command) {
        println!("{} is a shell builtin", command);
    } else {
        println!("{} invalid_command", command)
    }
}

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();

    let builtin: [&str; 3] = ["exit", "echo", "type"];

    loop {
        // Shell prefix output
        print!("$ ");
        io::stdout().flush().unwrap();

        // Get user input
        stdin
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // Input processing
        let trimmed_user_input = user_input.trim();

        if trimmed_user_input.is_empty() {
            user_input.clear();
            continue;
        }

        let mut tokens = trimmed_user_input.split_whitespace();
        let command = tokens.next().unwrap();
        let args: Vec<&str> = tokens.collect();

        // Command handling
        match command {
            "exit" => break,
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                if args.is_empty() {
                    user_input.clear();
                    continue;
                }
                check_type(args[0], builtin)
            }
            _ => println!("{}: command not found", command),
        }

        user_input.clear();
    }
}
