use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut line: String = String::new();
    let mut eof: bool = false;
    let mut v: Vec<u128> = Vec::new();

    while !eof {
        match handle.read_line(&mut line) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                line.pop();
                let mut iter = line.split_whitespace();
                v.push(iter.next().unwrap().parse().unwrap());
                v.push(iter.next().unwrap().parse().unwrap());
                v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                println!("{}", difference(v[1], v[0]));
                line.clear();
                v.clear();
            }
            Err(_err) => { panic!("ERROR"); }
        }
    }
}

fn difference(bigger: u128, smaller: u128) -> u128 {
    bigger - smaller
}