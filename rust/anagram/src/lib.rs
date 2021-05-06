use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let char_counts = counts(word);

    possible_anagrams
        .iter()
        .map(|w| (w, counts(*w)))
        .filter(|(_w, counts)| {
            subtract(&char_counts, counts).is_empty()
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

fn subtract(hm1: &HashMap<char, usize>, hm2: &HashMap<char, usize>) -> HashMap<char, usize> {
    println!("{:?}", hm1);
    println!("{:?}", hm2);
    let mut hm = hm1.clone();
    for (c, count) in hm2.iter() {
        let entry = hm.entry(*c).or_insert(0);
        *entry = entry.wrapping_sub(*count);
    }
    println!("{:?}", hm);
    println!();
    hm.drain().filter(|(_c, count)| *count != 0).collect()
}
