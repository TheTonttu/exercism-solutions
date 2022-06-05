use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut remaining_words: HashMap<&str, usize> = HashMap::new();

    for word in magazine {
        *remaining_words.entry(word).or_default() += 1;
    }

    for word in note {
        let entry = remaining_words.entry(word).or_default();
        if *entry > 0 {
            *entry -= 1;
        } else {
            return false;
        }
    }
    true
}
