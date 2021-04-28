const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
const VOWEL_SOUNDS: [[char; 2]; 4] = [['x', 'r'], ['y', 't'], ['X', 'R'], ['Y', 'T']];

pub fn translate(input: &str) -> String {
    if starts_with_vowel_sound(input) {
        [input, "ay"].concat()
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
