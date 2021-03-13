use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let unique_chars: HashSet<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    let char_numbers: HashMap<char, u8> = unique_chars
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i as u8))
        .collect();

    let words = input
        .split(' ')
        .filter(|element| element.chars().any(|c| c.is_alphabetic()));

    let words_as_numbers: Vec<u32> = words
        .filter_map(|w| {
            w.chars()
                .filter_map(|c| char_numbers.get(&c))
                .map(|n| n.to_string())
                .collect::<String>()
                .parse::<u32>()
                .ok()
        })
        .collect();

    println!("{:?}", words_as_numbers);

    let sum: u32 = words_as_numbers[..words_as_numbers.len() - 1].iter().sum();

    println!("{:?}", sum);

    let expected_sum: u32 = *words_as_numbers.last().unwrap();

    if sum == expected_sum {
        Some(char_numbers)
    } else {
        None
    }
}
