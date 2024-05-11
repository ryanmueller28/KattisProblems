use std::io;

fn main() {
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("FAILED TO READ INPUT");
    farewell(name.trim());
}

fn farewell(name: &str) {
    println!("Thank you, {name}, and farewell!");
}
