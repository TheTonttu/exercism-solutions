use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();

    let mut sorted_chars: Vec<char> = lowercase_word.chars().collect();
    sorted_chars.sort_unstable();

    println!("{:?}", sorted_chars);

    possible_anagrams
        .iter()
        .filter(|w| w.len() == word.len())
        .filter_map(|w| {
            let w_lower = w.to_lowercase();
            (w_lower != lowercase_word).then(|| (w, w_lower))
        })
        .map(|(w, w_lower)| {
            let mut sorted_w: Vec<char> = w_lower.chars().collect();
            sorted_w.sort_unstable();

            (w, sorted_w)
        })
        .filter(|(_w, s_w)| sorted_chars.iter().zip(s_w).all(|(c1, c2)| c1 == c2))
        .map(|(w, _s_w)| *w)
        .collect()
}
