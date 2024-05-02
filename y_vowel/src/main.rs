use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read input");
    println!(
        "{} {}",
        count_vowels(line.clone().as_str(), false),
        count_vowels(line.clone().as_str(), true)
    );
}

fn count_vowels(s: &str, include_y: bool) -> usize {
    s.chars().filter(|&c| is_vowel(c, include_y)).count()
}

fn is_vowel(c: char, include_y: bool) -> bool {
    if include_y {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'y'
    } else {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }
}
