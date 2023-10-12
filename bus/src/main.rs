use std::io;

fn main() {
    let mut nline: String = String::new();
    io::stdin().read_line(&mut nline).expect("Failed to read input");
    let t: i32 = nline.trim().parse().expect("Not a number");

    for _ in 0..t {
        let mut kline: String = String::new();
        io::stdin().read_line(&mut kline).expect("Failed to read input");
        let k: i32 = kline.trim().parse().expect("Not a number");
        println!("{}", stops_to_people(k))
    }
}

fn stops_to_people(stop_count: i32) -> i32 {
    let mut result: f64 = 0.0;
    for _ in 0..stop_count {
        result += 0.5;
        result *= 2.0;
    }

    result.floor() as i32
}