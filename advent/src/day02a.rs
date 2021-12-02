use algo_lib::io::input::Input;

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut pos = 0usize;
    let mut depth = 0usize;
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let dir: String = inp.read();
        let cur: usize = inp.read();
        match dir.as_str() {
            "forward" => pos += cur,
            "up" => depth -= cur,
            "down" => depth += cur,
            _ => unreachable!(),
        }
    }
    println!("{}", pos * depth);
}
