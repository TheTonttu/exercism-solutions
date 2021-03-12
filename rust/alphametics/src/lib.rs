use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let unique_chars: HashSet<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    let char_numbers: HashMap<char, u32> = unique_chars.iter().enumerate().map(|(i, c)| (*c,i as u32)).collect();

    let words = input.split(' ').filter(|element| element.len() > 2);

    let words_as_numbers = words.map(|w| w.chars().filter_map(|c| char_numbers.get(&c));

    unimplemented!("Solve the alphametic {:?}", input)
}
