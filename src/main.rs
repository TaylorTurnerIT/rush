#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};

#[derive(PartialEq)]
enum ControlFlow {
    Break,
    Continue,
    TypeBuiltin,
    TypeFile,
    TypeNotFound,
}

fn exec_command(command: &str, args: &Vec<&str>, is_type: bool) -> ControlFlow {
    match command {
        // -------------------------------------------------------------------------
        "exit" => {
            if !is_type {
                return ControlFlow::Break;
            }
        }
        // -------------------------------------------------------------------------
        "echo" => {
            if !is_type {
                println!("{}", args.join(" "))
            }
        }
        // -------------------------------------------------------------------------
        "type" => {
            if args.is_empty() {
                return ControlFlow::Continue;
            }
            if is_type {
                return ControlFlow::TypeBuiltin;
            }

            if exec_command(args[0], &args, true) == ControlFlow::TypeNotFound {
                println!("{}: not found", args[0])
            } else {
                println!("{} is a shell builtin", args[0])
            }
        }
        // -------------------------------------------------------------------------
        _ => {
            // check PATH
            let file_location = search_path(&command);
            // file found
            if file_location != "" {
                if is_type {
                    return ControlFlow::TypeFile;
                }
                println!("{} is {}", &command, file_location);
                return ControlFlow::Continue;
            }

            // not builtin AND not file
            if is_type {
                return ControlFlow::TypeNotFound;
            }
            println!("{}: command not found", command)
        }
    }

    if is_type {
        return ControlFlow::TypeBuiltin;
    } else {
        return ControlFlow::Continue;
    }
}

fn search_path(command: &str) -> String {
    // let command_path = env::var("PATH").unwrap();
    // println!("{}", env::current_dir().unwrap().display());
    // println!("{:#?}", env::var("PATH")); // this is how you get PATH

    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let mut current_path_dir = match path.read_dir() {
                    Ok(dir) => dir,
                    _ => continue, // ignore missing directories in path
                };
                let matched_file =
                    current_path_dir.find(|x| x.as_ref().unwrap().file_name() == command);
                match matched_file {
                    Some(Ok(f)) => {
                        // println!("{}", f.path().display()); //debug print
                        return f.path().display().to_string();
                    }
                    _ => continue,
                }
            }
        }
        None => println!("{key} is not defined in the environment."),
    }
    return "".to_string();
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
            ControlFlow::Break => break,
            ControlFlow::Continue => {
                user_input.clear();
                continue;
            }
            _ => {
                panic!(
                    "type command escaped with parameters: {}",
                    trimmed_user_input
                );
            }
        }
    }
}
