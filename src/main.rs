#[allow(unused_imports)]
use std::io::{self, Read, Write};

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        stdin
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let trimmed_user_input = user_input.trim();
        if !trimmed_user_input.is_empty() {
            println!("{}: command not found", trimmed_user_input);
        }

        user_input.clear();
    }
}
