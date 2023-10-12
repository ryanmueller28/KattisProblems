use std::io;

fn main() {
    let mut counter_attacks: Vec<&str> = Vec::with_capacity(6);
    counter_attacks.push("LBR");
    counter_attacks.push("LRB");
    counter_attacks.push("BLR");
    counter_attacks.push("BRL");
    counter_attacks.push("RLB");
    counter_attacks.push("RBL");

    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read input");
    let mut check_counter_string: String = String::new();
    let mut tmp_result: String = String::new();
    let mut result: String = String::new();
    for c in line.trim().chars() {
        check_counter_string.push(c);
        result.push(attack_to_counter(c));
        if counter_attacks.iter().any(|&s| s == check_counter_string.as_str()) {
            result.pop();
            result.pop();
            result.pop();
            result.push('C');
            check_counter_string.clear();
            tmp_result.clear();
        } else if check_counter_string.len() == 3 {
            check_counter_string.remove(0);
        }
    }
    println!("{}", result);
}


fn attack_to_counter(attack: char) -> char {
    match attack {
        'L' => 'H',
        'R' => 'S',
        'B' => 'K',
        _ => unreachable!("There should not be anything here")
    }
}

