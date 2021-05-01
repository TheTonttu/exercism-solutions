const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
const VOWEL_SOUNDS: [(char, char); 4] = [('x', 'r'), ('y', 't'), ('X', 'R'), ('Y', 'T')];
const CONSONANT_SOUNDS: [(char, char); 1] = [('q', 'u')];

pub fn translate(input: &str) -> String {
    input
        .split(' ')
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    if let Some(consonant_cluster) = take_start_consonant_cluster(word) {
        [
            word[consonant_cluster.len()..].to_string(),
            consonant_cluster,
            "ay".to_string(),
        ]
        .concat()
    } else {
        [word, "ay"].concat()
    }
}

fn take_start_consonant_cluster(input: &str) -> Option<String> {
    let mut consonant_cluster = String::new();

    let mut previous = None;
    let mut peekable_input = input.chars().peekable();
    while let Some(curr) = peekable_input.next() {
        if VOWELS.contains(&curr) {
            break;
        }

        if previous.is_none() {
            if let Some(&next) = peekable_input.peek() {
                if VOWEL_SOUNDS.contains(&(curr, next)) {
                    break;
                }
            }
        }

        consonant_cluster.push(curr);

        if let Some(&next) = peekable_input.peek() {
            if !previous.is_none() && next == 'y' || next == 'Y' {
                break;
            } else if CONSONANT_SOUNDS.contains(&(curr, next)) {
                consonant_cluster.push(next);
                // Skip iterating next char because it is now part of consonant cluster.
                peekable_input.next();
            }
        }

        previous = Some(curr);
    }

    (!consonant_cluster.is_empty()).then(|| consonant_cluster)
}
