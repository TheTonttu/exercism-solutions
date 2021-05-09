use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();

    let mut sorted_chars: Vec<char> = lowercase_word.chars().collect();
    sorted_chars.sort_unstable();

    println!("{:?}", sorted_chars);

    possible_anagrams
        .iter()
        .filter(|candidate| candidate.len() == word.len())
        .filter_map(|candidate| {
            let lowercase_candidate = candidate.to_lowercase();
            (lowercase_candidate != lowercase_word).then(|| (candidate, lowercase_candidate))
        })
        .map(|(candidate, lowercase_candidate)| {
            let mut sorted_candidate_chars: Vec<char> = lowercase_candidate.chars().collect();
            sorted_candidate_chars.sort_unstable();

            (candidate, sorted_candidate_chars)
        })
        .filter(|(_candidate, sorted_candidate_chars)| {
            sorted_chars
                .iter()
                .zip(sorted_candidate_chars)
                .all(|(c1, c2)| c1 == c2)
        })
        .map(|(candidate, _sorted_candidate_chars)| *candidate)
        .collect()
}
