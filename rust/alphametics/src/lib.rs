use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let unique_chars: HashSet<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    let number_permutations = (0u8..=9).permutations(unique_chars.len());

    let words: Vec<&str> = input
        .split(' ')
        .filter(|element| element.chars().any(|c| c.is_alphabetic()))
        .collect();

    // Gather characters that cannot be zero because they are in front of a word.
    let non_zero_chars: HashSet<char> = words
        .iter()
        .filter_map(|w| (w.len() >= 2).then(|| w.chars().next().unwrap()))
        .collect();

    // Brute force, yay!
    for numbers in number_permutations {
        let char_digit_designations: HashMap<char, u8> = unique_chars
            .iter()
            .zip(numbers)
            .map(|(c, n)| (*c, n))
            .collect();

        // Skip permutation if any non zero char is designated as zero
        if char_digit_designations
            .iter()
            .any(|(c, n)| *n == 0 && non_zero_chars.contains(c))
        {
            continue;
        }

        let word_digits = words
            .iter()
            .map(|w| {
                w.chars()
                    .filter_map(|c| char_digit_designations.get(&c).copied())
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<_>>();

        let words_as_numbers: Vec<u32> = word_digits
            .iter()
            .filter_map(|digits| {
                digits
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
            return Some(char_digit_designations);
        }
        //println!("wrong numbers: {:?}", words_as_numbers);
        //println!("expected sum: {:?}", sum);
    }

    None
}
