use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut remaining_words: HashMap<&str, usize> = HashMap::new();

    for word in magazine {
        let word_count = remaining_words.entry(word).or_default();
        *word_count += 1;
    }

    for word in note {
        let word_count = remaining_words.entry(word).or_default();
        if *word_count == 0 {
            return false;
        }
        *word_count -= 1;
    }
    true
}
