use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    println!("{}", greetings(line));
}


fn greetings(s: String) -> String {
    let mut result: String = String::new();
    let e_count = s.matches('e').count() * 2;
    result.push('h');
    for _ in 0..e_count {
        result.push('e');
    }
    result.push('y');
    result
}