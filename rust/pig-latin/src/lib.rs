const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
const VOWEL_SOUNDS: [[char; 2]; 4] = [['x', 'r'], ['y', 't'], ['X', 'R'], ['Y', 'T']];

pub fn translate(input: &str) -> String {
    if starts_with_vowel_sound(input) {
        [input, "ay"].concat()
    } else if starts_with_consonant_cluster_followed_by_y(input) {
        if let Some(consonant_count) = start_consonant_count(input) {
            let consonant_cluster = input[0..consonant_count].to_string();
            [
                input[consonant_count..].to_string(),
                consonant_cluster,
                "ay".to_string(),
            ]
            .concat()
        } else {
            String::from("oi!")
        }
    } else if let Some(consonant_count) = start_consonant_count(input) {
        let consonant_cluster = input[0..consonant_count].to_string();
        [
            input[consonant_count..].to_string(),
            consonant_cluster,
            "ay".to_string(),
        ]
        .concat()
    } else {
        match input.chars().next() {
            Some(first_char) => [
                input[1..].to_string(),
                first_char.to_string(),
                "ay".to_string(),
            ]
            .concat(),
            None => String::new(),
        }
    }
}

fn starts_with_vowel_sound(input: &str) -> bool {
    match input.chars().next() {
        Some(first_char) => {
            if VOWELS.contains(&first_char) {
                true
            } else if let Some(second_char) = input.chars().nth(1) {
                VOWEL_SOUNDS.contains(&[first_char, second_char])
            } else {
                false
            }
        }
        None => false,
    }
}

fn starts_with_consonant_cluster_followed_by_y(input: &str) -> bool {
    let mut consonant_count = 0;

    // TODO: Separate check for two character word (consonant + y) scenario
    for char in input.chars() {
        if VOWELS.contains(&char) {
            return false;
        } else if (char == 'y' || char == 'Y') && consonant_count >= 2 {
            return true;
        } else {
            consonant_count += 1;
        }
    }
    false
}

fn start_consonant_count(input: &str) -> Option<usize> {
    let count = input
        .chars()
        .take_while(|c| !(VOWELS.contains(&c) || *c == 'y' || *c == 'Y'))
        .count();

    (count >= 1).then(|| count)
}
