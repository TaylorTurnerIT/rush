#[allow(unused_imports)]
use std::io::{self, Read, Write};
use std::ops::ControlFlow;

fn exec_command(command: &str, args: &Vec<&str>, is_type: bool) -> ControlFlow<()> {
    match command {
        "exit" => {
            if !is_type {
                return ControlFlow::Break(());
            }
        }
        "echo" => {
            if !is_type {
                println!("{}", args.join(" "))
            }
        }
        "type" => {
            if args.is_empty() || is_type {
                return ControlFlow::Continue(());
            }

            if exec_command(args[0], &args, true) == ControlFlow::Break(()) {
                println!("{}: not found", args[0])
            } else {
                println!("{} is a shell builtin", args[0])
            }
        }
        _ => {
            if !is_type {
                println!("{}: command not found", command)
            }
        }
    }
    return ControlFlow::Continue(());
}

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
        match exec_command(&command, &args, false) {
            ControlFlow::Break(()) => break,
            ControlFlow::Continue(()) => {
                user_input.clear();
                continue;
            }
        }
    }
}
