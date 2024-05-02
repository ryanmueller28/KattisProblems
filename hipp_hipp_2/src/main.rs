use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let binding = input.clone();
    let name = binding.trim();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n_times = input.trim().parse().expect("Failed to read a number");
    for _ in 0..n_times {
        println!("Hipp hipp hurra, {}!", name)
    }
}


