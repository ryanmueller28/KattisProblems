use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read input");
    let mut iter = line.split_whitespace();
    let x_juncs: i32 = iter.next().unwrap().parse().unwrap();
    let y_juncs: i32 = iter.next().unwrap().parse().unwrap();
    println!("{}", loop_possible_and_print(y_juncs));
}

#[warn(unused_variables)]
fn loop_possible_and_print(y_juncs: i32) -> String {
    if y_juncs % 2 == 0 {
        "possible".to_string()
    } else {
        "impossible".to_string()
    }
}