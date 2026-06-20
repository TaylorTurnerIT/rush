#[allow(unused_imports)]
use std::io::{self, Read, Write};

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        // Shell prefix output
        print!("$ ");
        io::stdout().flush().unwrap();

        // Get user input
        stdin
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // Input handling
        let trimmed_user_input = user_input.trim();

        if trimmed_user_input.is_empty() {
            user_input.clear();
            continue;
        }

        match trimmed_user_input {
            "exit" => break,
            _ => println!("{}: command not found", trimmed_user_input),
        }

        user_input.clear();
    }
}
