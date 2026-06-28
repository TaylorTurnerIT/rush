use std::env;

fn main() {
    let message: Vec<String> = std::env::args().skip(1).collect();
    print!("{:#?}", message);
    return;
}
