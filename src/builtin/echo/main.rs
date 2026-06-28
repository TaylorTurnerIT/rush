use std::env;

fn main() {
    let message: String = env::args().collect();
    print!("{}", message);
    return;
}
