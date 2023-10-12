use std::io;
use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();
    let mut eof = false;

    while !eof {
        match handle.read_line(&mut line) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                line.pop();
                println!("{}", calculate_cost(line.trim()));
                line.clear();
            }
            Err(_err) => { panic!("ERROR"); }
        }
    }
}

fn calculate_cost(s: &str) -> u32 {
    let mut cost_map = fill_hash_map();
    let mut out: u32 = 0;
    for c in s.chars().to_owned() {
        let tmp = cost_map.get(&c);
        out += match tmp {
            Some(tmp) => tmp,
            None => &0
        }
    }
    out
}

fn fill_hash_map() -> HashMap<char, u32> {
    let mut result: HashMap<char, u32> = HashMap::new();

    result.insert(' ', 0);
    result.insert('!', 9);
    result.insert('"', 6);
    result.insert('#', 24);
    result.insert('$', 29);
    result.insert('%', 22);
    result.insert('&', 24);
    result.insert('\'', 3);
    result.insert('(', 12);
    result.insert(')', 12);
    result.insert('*', 17);
    result.insert('+', 13);
    result.insert(',', 7);
    result.insert('-', 7);
    result.insert('.', 4);
    result.insert('/', 10);
    result.insert('0', 22);
    result.insert('1', 19);
    result.insert('2', 22);
    result.insert('3', 23);
    result.insert('4', 21);
    result.insert('5', 27);
    result.insert('6', 26);
    result.insert('7', 16);
    result.insert('8', 23);
    result.insert('9', 26);
    result.insert(':', 8);
    result.insert(';', 11);
    result.insert('<', 10);
    result.insert('=', 14);
    result.insert('>', 10);
    result.insert('?', 15);
    result.insert('@', 32);
    result.insert('A', 24);
    result.insert('B', 29);
    result.insert('C', 20);
    result.insert('D', 26);
    result.insert('E', 26);
    result.insert('F', 20);
    result.insert('G', 25);
    result.insert('H', 25);
    result.insert('I', 18);
    result.insert('J', 18);
    result.insert('K', 21);
    result.insert('L', 16);
    result.insert('M', 28);
    result.insert('N', 25);
    result.insert('O', 26);
    result.insert('P', 23);
    result.insert('Q', 31);
    result.insert('R', 28);
    result.insert('S', 25);
    result.insert('T', 16);
    result.insert('U', 23);
    result.insert('V', 19);
    result.insert('W', 26);
    result.insert('X', 18);
    result.insert('Y', 14);
    result.insert('Z', 22);
    result.insert('[', 18);
    result.insert('\\', 10);
    result.insert(']', 18);
    result.insert('^', 7);
    result.insert('_', 8);
    result.insert('`', 3);
    result.insert('a', 23);
    result.insert('b', 25);
    result.insert('c', 17);
    result.insert('d', 25);
    result.insert('e', 23);
    result.insert('f', 18);
    result.insert('g', 30);
    result.insert('h', 21);
    result.insert('i', 15);
    result.insert('j', 20);
    result.insert('k', 21);
    result.insert('l', 16);
    result.insert('m', 22);
    result.insert('n', 18);
    result.insert('o', 20);
    result.insert('p', 25);
    result.insert('q', 25);
    result.insert('r', 13);
    result.insert('s', 21);
    result.insert('t', 17);
    result.insert('u', 17);
    result.insert('v', 13);
    result.insert('w', 19);
    result.insert('x', 13);
    result.insert('y', 24);
    result.insert('z', 19);
    result.insert('{', 18);
    result.insert('|', 12);
    result.insert('}', 18);
    result.insert('~', 9);
    result
}