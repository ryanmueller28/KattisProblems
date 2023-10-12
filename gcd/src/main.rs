use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    println!("{}", gcd(a, b));
}

fn gcd(m: i32, n: i32) -> i32 {
    if m == 0 {
        n.abs()
    } else {
        gcd(n % m, m)
    }
}