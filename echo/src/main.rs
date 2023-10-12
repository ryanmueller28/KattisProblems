use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    println!("{}", echo(line));
}

fn echo(s: String) -> String {
    s.repeat(3)
}
