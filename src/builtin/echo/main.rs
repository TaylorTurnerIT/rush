use std::env;

fn main() {
    let message: String = env::args().next().unwrap().collect();
    print!("{}", message);
    return;
}
