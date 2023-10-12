use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read");
    let mut iter = line.split_whitespace();
    let a: u128 = iter.next().unwrap().parse().unwrap();
    let b: u128 = iter.next().unwrap().parse().unwrap();
    let c: u128 = iter.next().unwrap().parse().unwrap();
    let d: u128 = iter.next().unwrap().parse().unwrap();
    println!("{}", bizz_fuzz_lcm(a, b, c, d));
}

fn bizz_fuzz_lcm(a: u128, b: u128, c: u128, d: u128) -> u128 {
    let x: u128 = lcm(c, d);
    b / x - (a -1) / x
}

fn gcd(x: u128, y: u128) -> u128 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn lcm(x: u128, y: u128) -> u128 {
    x * y / gcd(x, y)
}