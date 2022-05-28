use if_chain::if_chain;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let unique_chars: HashSet<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    let words: Vec<&str> = input
        .split(' ')
        .filter(|element| element.chars().any(|c| c.is_alphabetic()))
        .collect();

    if is_unsolvable(&words) {
        return None;
    }

    let mut preassigned_designations: HashMap<char, u8> = HashMap::new();

    let sum_word = words.last().unwrap();
    let addends = &words[..words.len() - 1];
    // If sum is longer than any of the addends then the first letter of the sum is 1 from carry.
    if sum_word.len() > addends.iter().map(|w| w.len()).max().unwrap() {
        preassigned_designations.insert(sum_word.chars().nth(0).unwrap(), 1);
    }

    // Gather characters that cannot be zero because they are in front of a word.
    let non_zero_chars: HashSet<char> = words
        .iter()
        .filter_map(|w| (w.len() >= 2).then(|| w.chars().next().unwrap()))
        .collect();

    // If there is only one character that can be zero then mark it for later use.
    let only_zero_char = (unique_chars.len() == non_zero_chars.len() + 1).then(|| {
        unique_chars
            .iter()
            .find(|c| !non_zero_chars.contains(*c))
            .copied()
            .unwrap()
    });

    let number_permutations = (0u8..=9).permutations(unique_chars.len());
    // Brute force, yay!
    for numbers in number_permutations {
        let char_digit_designations: HashMap<char, u8> = unique_chars
            .iter()
            .zip(numbers)
            .map(|(c, n)| (*c, n))
            .collect();

        if_chain! {
            // If there can be only one char designated as zero
            if let Some(zero_char) = only_zero_char;
            if let Some(digit) = char_digit_designations.get(&zero_char);
            // and it has not been designated as zero
            if *digit != 0;
            then {
                // then skip permutation
                continue;
            }
        }

        if char_digit_designations
            .iter()
            .any(|(c, n)| {
                // Skip permutation if any non zero char is designated as zero
                *n == 0 && non_zero_chars.contains(c)
                // or preassigned designations do not match
                || preassigned_designations.get(c).map_or(false, |dn| n != dn)
            })
        {
            continue;
        }

        let words_as_numbers: Vec<u64> = words
            .iter()
            .map(|w| {
                w.chars()
                    // char to digit
                    .filter_map(|c| char_digit_designations.get(&c))
                    // digits to number
                    .fold(0u64, |acc, d| acc * 10 + (*d as u64))
            })
            .collect();

        let sum: u64 = words_as_numbers[..words_as_numbers.len() - 1].iter().sum();

        let expected_sum: u64 = *words_as_numbers.last().unwrap();

        if sum == expected_sum {
            return Some(char_digit_designations);
        }
    }
    None
}

fn is_unsolvable(words: &Vec<&str>) -> bool {
    match &words[..] {
        // A == B
        [first, second] => first != second,
        // AAA + BB == CC
        [addends @ .., sum] => addends.iter().any(|w| w.len() > sum.len()),
        _ => false,
    }
}
