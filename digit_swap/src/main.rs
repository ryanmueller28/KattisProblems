use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    println!("{}", swap(line.as_str()));
}

fn swap(s: &str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.swap(0 as usize, 1 as usize);
    chars.into_iter().collect()
}
