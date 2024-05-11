use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("FAILED TO READ USER INPUT");
    println!("{}", backspace(input.trim(), '<'));
}

fn backspace(s: &str, check_char: char) -> String {
    let mut result: String = String::new();

    for c in s.chars() {
        result.push(c);
        if c == check_char {
            result.pop();
            result.pop();
        }
    }

    result
}
