use std::env;
use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;

fn main() {
    // Collect args
    let command = match std::env::args().nth(1) {
        Some(c) => c,
        None => {
            return;
        }
    };

    // Fetch PATH from the OS
    let paths;
    match env::var_os("PATH") {
        Some(val) => paths = val,
        None => {
            println!("PATH is not defined in the environment.");
            return;
        }
    };

    // Iterate through each directory in PATH
    let mut dir_files;
    for dir in env::split_paths(&paths) {
        match dir.read_dir() {
            Ok(f) => dir_files = f,
            Err(_e) => return,
        }

        match dir_files.find(|x| x.as_ref().unwrap().file_name() == OsString::from(&command)) {
            Some(Ok(f)) => {
                if f.metadata().unwrap().permissions().mode() & 0o111 != 0 {
                    println!("{} is {}", command, f.path().display().to_string());
                    return;
                }
            }
            _ => (),
        }
    }
    println!("{}: not found", command)
}
