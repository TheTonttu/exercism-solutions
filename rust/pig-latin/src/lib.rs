const PIG_LATIN_SOUND: &str = "ay";
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const VOWEL_SOUNDS: [(char, char); 2] = [('x', 'r'), ('y', 't')];
const CONSONANT_SOUNDS: [(char, char); 1] = [('q', 'u')];

pub fn translate(input: &str) -> String {
    input
        .to_lowercase()
        .split(' ')
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    let consonant_cluster = take_start_consonant_cluster(word);

    [
        word[consonant_cluster.len()..].to_string(),
        consonant_cluster,
        PIG_LATIN_SOUND.to_string(),
    ]
    .concat()
}

fn take_start_consonant_cluster(word: &str) -> String {
    let mut consonant_cluster = String::new();

    let mut previous = None;
    let mut peekable_word_iterator = word.chars().peekable();
    while let Some(current) = peekable_word_iterator.next() {
        if VOWELS.contains(&current) {
            break;
        }

        let is_starting_char = previous.is_none();
        if is_starting_char {
            if let Some(&next) = peekable_word_iterator.peek() {
                if VOWEL_SOUNDS.contains(&(current, next)) {
                    break;
                }
            }
        } else if current == 'y' {
            // Y is considered vowel if it is preceded by consonant cluster.
            break;
        }

        consonant_cluster.push(current);

        if let Some(&next) = peekable_word_iterator.peek() {
            if CONSONANT_SOUNDS.contains(&(current, next)) {
                consonant_cluster.push(next);
                // Skip iterating next char because it is now part of consonant cluster.
                peekable_word_iterator.next();
            }
        }

        previous = Some(current);
    }

    consonant_cluster
}
