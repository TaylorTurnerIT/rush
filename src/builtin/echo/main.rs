fn main() {
    let message: Vec<String> = std::env::args().skip(1).collect();
    println!("{}", message.join(" "));
    return;
}
