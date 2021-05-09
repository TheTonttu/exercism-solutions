use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();

    let sorted_chars = {
        let mut chars: Vec<char> = lowercase_word.chars().collect();
        chars.sort_unstable();
        chars
    };

    let mut anagrams = HashSet::new();
    for candidate in possible_anagrams {
        if candidate.len() != word.len() {
            continue;
        }
        let lowercase_candidate = candidate.to_lowercase();
        if lowercase_candidate == lowercase_word {
            continue;
        }

        let sorted_candidate_chars = {
            let mut chars: Vec<char> = lowercase_candidate.chars().collect();
            chars.sort_unstable();
            chars
        };

        if sorted_chars
            .iter()
            .zip(sorted_candidate_chars)
            .all(|(c1, c2)| *c1 == c2)
        {
            anagrams.insert(*candidate);
        }
    }

    anagrams
}
