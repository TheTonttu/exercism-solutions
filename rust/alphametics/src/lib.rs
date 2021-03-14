use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let unique_chars: HashSet<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    let number_permutations = (0u8..=9).permutations(unique_chars.len());

    let words: Vec<&str> = input
        .split(' ')
        .filter(|element| element.chars().any(|c| c.is_alphabetic()))
        .collect();

    // Brute force, yay!
    for numbers in number_permutations {
        let char_number_designations: HashMap<char, u8> = unique_chars
            .iter()
            .zip(numbers)
            .map(|(c, i)| (*c, i))
            .collect();

        let collected_word_numbers = words
            .iter()
            .map(|w| {
                w.chars()
                    .filter_map(|c| char_number_designations.get(&c))
                    .copied()
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<_>>();

        if has_any_leading_zeroes(&collected_word_numbers) {
            continue;
        }

        let words_as_numbers: Vec<u32> = collected_word_numbers
            .iter()
            .filter_map(|numbers| {
                numbers
                    .iter()
                    .filter_map(|n| std::char::from_digit(*n as u32, 10))
                    // TODO: Skip intermediate String collection
                    .collect::<String>()
                    .parse::<u32>()
                    .ok()
            })
            .collect();

        let sum: u32 = words_as_numbers[..words_as_numbers.len() - 1].iter().sum();

        let expected_sum: u32 = *words_as_numbers.last().unwrap();

        if sum == expected_sum {
            return Some(char_number_designations);
        }
        //println!("wrong numbers: {:?}", words_as_numbers);
        //println!("expected sum: {:?}", sum);
    }

    None
}

fn has_any_leading_zeroes(number_slice: &[Vec<u8>]) -> bool {
    number_slice
        .iter()
        .any(|wn| wn.len() >= 2 && wn.starts_with(&[0]))
}
