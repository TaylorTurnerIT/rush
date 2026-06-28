#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::process::Command;

fn exec_command(command: &str, args: &Vec<&str>) {
    // Check if builtin
    let exe_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let builtin_command = exe_dir.join(command);
    let run_builtin = Command::new(builtin_command).args(args).spawn();
    match run_builtin {
        Ok(mut child) => {
            child.wait().unwrap();
            return;
        }
        Err(_e) => (),
    }

    // Check if in PATH
    let run = Command::new(command).args(args).spawn();
    match run {
        Ok(mut child) => {
            child.wait().unwrap();
            return;
        }
        Err(_e) => println!("{}: command not found", command),
    }
    return;
}

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        // Shell prefix output
        print!("$ ");
        match io::stdout().flush() {
            Ok(_t) => (),
            Err(e) => println!("Error: {}", e),
        }

        // Get user input
        match stdin.read_line(&mut user_input) {
            Ok(_t) => (),
            Err(e) => println!("Error: {}", e),
        }

        // Input processing
        let trimmed_user_input = user_input.trim();
        if trimmed_user_input.is_empty() {
            user_input.clear();
            continue;
        }

        let mut tokens = trimmed_user_input.split_whitespace();
        let command;
        match tokens.next() {
            Some(t) => command = t,
            None => continue,
        }
        let args: Vec<&str> = tokens.collect();

        // Command handling
        if command == "exit" {
            break;
        }

        exec_command(&command, &args);
        user_input.clear();
    }
}
