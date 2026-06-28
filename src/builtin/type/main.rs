use std::env;
use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn main() {
    // Collect args
    let command = match std::env::args().nth(1) {
        Some(c) => c,
        None => {
            return;
        }
    };

    // Fetch PATH from the OS
    let exe_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();

    let raw_path = env::var_os("PATH").unwrap_or_default();
    let mut search_dirs: Vec<PathBuf> = vec![exe_dir.clone()];
    search_dirs.extend(env::split_paths(&raw_path));
    let paths = env::join_paths(search_dirs).unwrap();

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
                    if f.path().parent().unwrap() == exe_dir {
                        println!("{} is a shell builtin", command);
                        return;
                    }
                    println!("{} is {}", command, f.path().display().to_string());
                    return;
                }
            }
            _ => (),
        }
    }
    println!("{}: not found", command)
}
