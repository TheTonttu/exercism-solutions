use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let chars_present: HashSet<_> = sentence.chars().map(|c| c.to_ascii_lowercase()).collect();

    ('a'..='z').all(|c| chars_present.contains(&c))
}
