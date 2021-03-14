use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let unique_chars: HashSet<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    let number_permutations = (0u8..=9).permutations(unique_chars.len());

    // Brute force, yay!
    for numbers in number_permutations {
        let char_numbers: HashMap<char, u8> = unique_chars
            .iter()
            .zip(numbers)
            .map(|(c, i)| (*c, i))
            .collect();

        let words = input
            .split(' ')
            .filter(|element| element.chars().any(|c| c.is_alphabetic()));

        // TODO: Skip the permutation if any of the converted words have leading zeroes.

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

        let sum: u32 = words_as_numbers[..words_as_numbers.len() - 1].iter().sum();

        let expected_sum: u32 = *words_as_numbers.last().unwrap();

        if sum == expected_sum {
            return Some(char_numbers);
        }
        println!("wrong numbers: {:?}", words_as_numbers);
        println!("expected sum: {:?}", sum);
    }

    None
}
