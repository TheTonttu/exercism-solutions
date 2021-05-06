use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let char_counts = counts(word);

    possible_anagrams
        .iter()
        .map(|w| (w, counts(*w)))
        .filter(|(_w, counts)| {
            char_counts
                .iter()
                .all(|(c, count)| counts.get(c).map(|count2| count == count2) == Some(true))
        })
        .map(|(w, _counts)| *w)
        .collect()
}

fn counts(word: &str) -> HashMap<char, usize> {
    word.chars().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0) += 1;
        hm
    })
}
