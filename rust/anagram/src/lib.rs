use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_chars = get_sorted_chars(&lowercase_word);

    let mut anagrams = HashSet::new();
    for candidate in possible_anagrams {
        if candidate.len() != word.len() {
            continue;
        }
        let lowercase_candidate = candidate.to_lowercase();
        if lowercase_candidate == lowercase_word {
            continue;
        }

        let sorted_candidate_chars = get_sorted_chars(&lowercase_candidate);

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

fn get_sorted_chars(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars
}
