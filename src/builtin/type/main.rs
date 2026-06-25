use std::env;
use std::ffi::OsStr;
use std::os::unix::fs::PermissionsExt;

fn main() {
    // Collect args
    let args: Vec<String> = std::env::args().collect();
    let command: &OsStr = OsStr::new(&args[1]);

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

        match dir_files.find(|x| x.as_ref().unwrap().file_name() == command) {
            Some(Ok(f)) => {
                if f.metadata().unwrap().permissions().mode() & 0o111 != 0 {
                    println!("{}", f.path().display().to_string());
                }
            }
            _ => return,
        }
    }
}

// fn search_path(command: &str) -> String {
// let command_path = env::var("PATH").unwrap();
// println!("{}", env::current_dir().unwrap().display());
// println!("{:#?}", env::var("PATH")); // this is how you get PATH

// Add builtin to PATH variable
// todo!();

// for path in env::split_paths(&paths) {
//     let mut current_path_dir = match path.read_dir() {
//         Ok(dir) => dir,
//         _ => continue,
//     };

//     let matched_file =
//         current_path_dir.find(|x| x.as_ref().unwrap().file_name() == command);
//     match matched_file {
//         Some(Ok(f)) => {
//             if f.metadata().unwrap().permissions().mode() & 0o111 != 0 {
//                 return f.path().display().to_string();
//             }
//             continue;
//         }
//         _ => continue,
//     }
// }
// }
// }
// }
