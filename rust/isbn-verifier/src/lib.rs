use std::iter;

const NUMBER_BASE: u32 = 10;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    const ISBN10_VALID_LENGTH: usize = 10;
    const CHECK_DIGIT_INDEX: usize = 9;
    const CHECK_MODULUS: u32 = 11;

    let stripped: String = isbn
        .chars()
        .filter(|c| ('0'..='9').contains(c) || *c == 'X')
        .collect();

    println!("{:?}", stripped);

    if stripped.len() != ISBN10_VALID_LENGTH {
        return false;
    }
    if let Some(x_index) = stripped.find('X') {
        if x_index != CHECK_DIGIT_INDEX {
            return false;
        }
    }

    if let Some(check_digit) = stripped
        .chars()
        .nth(CHECK_DIGIT_INDEX)
        .and_then(parse_check_digit)
    {
        let sum: u32 = stripped[..CHECK_DIGIT_INDEX]
            .chars()
            .filter_map(|c| c.to_digit(NUMBER_BASE))
            .chain(iter::once(check_digit))
            // Include multipliers
            .zip((1..=(stripped.len() as u32)).rev())
            .map(|(value, multiplier)| value * multiplier)
            .sum();

        sum % CHECK_MODULUS == 0
    } else {
        false
    }
}

fn parse_check_digit(char: char) -> Option<u32> {
    match char {
        '0'..='9' => char.to_digit(NUMBER_BASE),
        'X' => Some(10),
        _ => None,
    }
}
