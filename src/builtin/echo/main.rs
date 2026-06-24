use std::env;

fn main() {
    let message = match env::args().next() {
        Some(msg) => msg,
        None => "".to_string(),
    };
    print!("{}", message);
    return;
}
