use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let inputs: Vec<u32> = line.split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();
    println!("{}", alex_or_barb(inputs[0], inputs[1], inputs[2]));
    // let result: &str = if is_valid_eye(&line) { "correct" } else { "fix" };
    // println!("{}", result);
}

fn alex_or_barb(k: u32, m: u32, n: u32) -> &'static str {
    if k % (m + n) < m { "Barb" } else { "Alex" }
}

