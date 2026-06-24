enum ControlFlow {
    Break,
    Continue,
}

fn main(command: &str, args: &Vec<&str>) -> ControlFlow {
    return ControlFlow::Break;
}
