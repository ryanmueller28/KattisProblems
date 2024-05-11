use std::io::{self};

fn main() {
    let mut sentence: String = String::new();
    io::stdin().read_line(&mut sentence).expect("FAILED TO READ INPUT");
    println!("{}", count_vowels(sentence.trim()));
}

fn count_vowels(s: &str) -> usize {
    s.chars().into_iter().filter(|c| is_vowel(*c)).count()
}

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
