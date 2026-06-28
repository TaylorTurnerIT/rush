use std::env;

fn main() {
    let message: String = std::env::args().skip(1).collect();
    print!("{:#?}", message);
    return;
}
