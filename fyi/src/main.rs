use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let slice = &line.as_str()[..3];
    println!("{}", slice);
}

fn fyi(s: &str) -> i32 {
    match s {
        "555" => 1,
        _ => 0
    }
}