use std::io;

fn main() {
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read user input");
    let banned_letters = user_input.clone();
    user_input.clear();
    io::stdin().read_line(&mut user_input).expect("Failed to read user input");
    let msg_words = user_input.split_ascii_whitespace().collect();
    println!("{}", bannord(banned_letters.trim(), msg_words));
}

fn bannord(s: &str, m: Vec<&str>) -> String {
    let mut result: String = String::new();
    let banned: Vec<char> = s.chars().collect();
    let mut found = false;
    for word in m {
        for c in banned.iter() {
            match word.find(*c) {
                Some(_) => {
                    found = true;
                    break;
                },
                None => found = false, 
            }
        }

        if found {
            result.push_str("*".repeat(word.len()).as_str());
        } else {
            result.push_str(word);
        }
        result.push(' ');
    }

    result.trim().to_string()
}

