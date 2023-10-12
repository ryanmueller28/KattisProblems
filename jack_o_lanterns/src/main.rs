use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();
    println!("{}", jack_o_lanterns(a, b, c));
}

fn jack_o_lanterns(eyes: i32, noses: i32, mouths: i32) -> i32 {
    eyes * noses * mouths
}