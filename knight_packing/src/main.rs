use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let trimmed = line.trim();
    let n: i32 = trimmed.parse().unwrap();
    println!("{}", knight_packing(n));
}

fn knight_packing(n: i32) -> String {
    if n % 2 == 0 { "second".to_string() } else {"first".to_string()}
}