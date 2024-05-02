use std::collections::HashMap;

fn map_alphabet() -> HashMap<char, &'static str> {
    let mut result = HashMap::new();
    result.insert('a', "@");
    result.insert('A', "@");
    
    result.insert('b', "8");
    result.insert('B', "8");
    
    result.insert('c', "(");
    result.insert('C', "(");
    
    result.insert('d', "|)");
    result.insert('D', "|)");
    
    result.insert('e', "3");
    result.insert('E', "3");
    
    result.insert('f', "#");
    result.insert('F', "#");
    
    result.insert('g', "6");
    result.insert('G', "6");
    
    result.insert('h', "[-]");
    result.insert('H', "[-]");
    
    result.insert('i', "|");
    result.insert('I', "|");
    
    result.insert('j', "_|");
    result.insert('J', "_|");

    result.insert('k', "|<");
    result.insert('K', "|<");
    
    result.insert('l', "1");
    result.insert('L', "1");
    
    result.insert('m', "[]\\/[]");
    result.insert('M', "[]\\/[]");
    
    result.insert('n', "[]\\[]");
    result.insert('N', "[]\\[]");
    
    result.insert('o', "0");
    result.insert('O', "0");
    
    result.insert('p', "|D");
    result.insert('P', "|D");
    
    result.insert('q', "(,)");
    result.insert('Q', "(,)");
    
    result.insert('r', "|Z");
    result.insert('R', "|Z");
    
    result.insert('s', "$");
    result.insert('S', "$");
    
    result.insert('t', "']['");
    result.insert('T', "']['");
    
    result.insert('u', "|_|");
    result.insert('U', "|_|");
    
    result.insert('v', "\\/");
    result.insert('V', "\\/");
    
    result.insert('w', "\\/\\/");
    result.insert('W', "\\/\\/");
    
    result.insert('x', "}{");
    result.insert('X', "}{");
    
    result.insert('y', "`/");
    result.insert('Y', "`/");
    
    result.insert('z', "2");
    result.insert('Z', "2");
    
    result
}

fn word_to_leet_code(word: &str) -> String {
    let mut result: String = String::new(); 
    let alpha_map = map_alphabet();

    for c in word.chars() {
        if let Some(v) = alpha_map.get(&c) {
            result.push_str(v);
        } else {
            result.push(c);
        }
    } 
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_works() {
        let result = String::from("@11 `/0|_||Z 8@$3 @|Z3 8310[]\\[]6 ']['0 |_|$.");
        let actual = word_to_leet_code("All your base are belong to us.");
        assert_eq!(result, actual);
    }

    #[test]
    fn example_two_works() {
        let result = String::from("\\/\\/[-]@'][''$ ']['[-]3 #|Z3(,)|_|3[]\\[](`/, |<3[]\\[][]\\[]3']['[-]?");
        let actual = word_to_leet_code("What's the Frequency, Kenneth?");
        assert_eq!(result, actual);
    }
}
