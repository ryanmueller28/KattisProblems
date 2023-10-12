use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read input!");
    println!("{}", canadians_eh(line));
}

fn canadians_eh(s: String) -> String {
    let v: Vec<&str> = s.trim().split_whitespace().collect();
    let l = v.len();
    match v[l - 1] {
        "eh?" => "Canadian!".to_string(),
        _ => "Imposter!".to_string()
    }
}