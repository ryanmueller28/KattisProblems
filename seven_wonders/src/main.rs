use std::io;
use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read input");
    let mut s = line.to_string();
    let len = s.len();
    s.truncate(len - 1);
    println!("{}", seven_wonders(s.as_str()));
}


fn seven_wonders(hand: &str) -> i32 {
    let mapped_hand = count(hand);
    let mut result: i32 = 0;
    let len_hand = mapped_hand.len();
    let mut min_value = i32::MAX;
    for (_, val) in mapped_hand {
        if min_value > val && len_hand == 3 {
            min_value = val;
        }
        result += val.pow(2);
    }
    if len_hand == 3 {
        result += min_value * 7;
    }
    result
}

fn count(input: &str) -> HashMap<char, i32> {
    // convert string to char array
    let char_arr: Vec<char> = input.to_lowercase().chars().collect();
    let mut counts: HashMap<char, i32> = HashMap::new();

    // iterate over string, and if HashMap<C> has a value, +1 else put new value
    for i in char_arr {
        *counts.entry(i).or_insert(0) += 1;
    }

    // return result
    counts
}