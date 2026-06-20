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

        println!("{}: command not found", user_input.trim());
        user_input.clear();
    }
}

/*
 * In this stage, you'll implement support for printing error messages for invalid commands.
 *
 *  Example:
 *  $ xyz
 *  xyz: command not found
 *
 *  Your program should:
 *
 *  Display the prompt $ (keep the code from the previous stage)
 *  Read the user's input
 *  Print an error message in exactly this format: {command}: command not found
 *      e.g. if the user types xyz, print xyz: command not found
 *
 *  For now, we'll treat all commands as "invalid". In later stages we'll handle executing "valid" commands like echo, cd etc.
 *
 */
