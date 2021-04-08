const X_NUMBER: u32 = 10;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    const VALID_ISBN10_NUMBER_COUNT: usize = 10;
    const CHECK_DIGIT_INDEX: usize = 9;
    const CHECK_MODULUS: u32 = 11;

    let isbn_numbers: Vec<u32> = isbn.chars().filter_map(parse_isbn_number).collect();

    println!("{:?}", isbn_numbers);

    if isbn_numbers.len() != VALID_ISBN10_NUMBER_COUNT {
        return false;
    }

    if let Some(x_index) = isbn_numbers.iter().position(|v| *v == X_NUMBER) {
        if x_index != CHECK_DIGIT_INDEX {
            return false;
        }
    }

    let sum: u32 = isbn_numbers
        .iter()
        // Include multipliers
        .zip((1..=(isbn_numbers.len() as u32)).rev())
        .map(|(value, multiplier)| value * multiplier)
        .sum();

    sum % CHECK_MODULUS == 0
}

fn parse_isbn_number(char: char) -> Option<u32> {
    const NUMBER_BASE: u32 = 10;

    match char {
        '0'..='9' => char.to_digit(NUMBER_BASE),
        'X' => Some(X_NUMBER),
        _ => None,
    }
}
