use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    println!("{}", eye_of_sauron(line.trim().to_string()));
}

fn eye_of_sauron(s: String) -> String {
    let split: Vec<_> = s.split("()").collect();
    if split[0].len() == split[1].len() { "correct".to_string() } else {"fix".to_string()}
}