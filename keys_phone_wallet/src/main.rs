use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Failed to read input");
    let n_times = line.trim().parse().expect("Input was not a number");
    let mut prepped_items: Vec<String> = Vec::new();
    for _ in 0..n_times {
        let mut item: String = String::new();
        io::stdin().read_line(&mut item).expect("Failed to read input");
        prepped_items.push(item.trim().to_string());
    }

    let result = keys_wallet_phone(prepped_items.clone());
    for i in result.iter(){
        println!("{}", i);
    }
}

fn keys_wallet_phone(prepared: Vec<String>) -> Vec<String> {
    let needed = vec!["keys", "phone", "wallet"];
    let mut result: Vec<String> = Vec::new();
    for need in needed.iter() {
        if !prepared.contains(&need.to_string()) {
          result.push(need.to_string());  
        }
    }

    if result.len() == 0 { 
        result.push("ready".to_string());
    }
    result
}
