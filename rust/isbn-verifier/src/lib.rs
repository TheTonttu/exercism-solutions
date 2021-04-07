/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    const ISBN_VALID_LENGTH: usize = 10;
    const CHECK_DIGIT_INDEX: usize = 9;

    let stripped: String = isbn
        .chars()
        .filter(|c| ('0'..='9').contains(c) || *c == 'X')
        .collect();

    println!("{:?}", stripped);

    if stripped.len() != ISBN_VALID_LENGTH {
        return false;
    }
    if let Some(x_index) = stripped.find('X') {
        if x_index != 9 {
            return false;
        }
    }

    let mut values: Vec<u32> = stripped[..9]
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    println!("{:?}", values);

    if let Some(check_val) = stripped.chars().nth(CHECK_DIGIT_INDEX) {
        let check_digit = match check_val {
            '0'..='9' => check_val.to_digit(10).unwrap(),
            'X' => 10,
            _ => {
                return false;
            }
        };
        println!("check digit: {}", check_digit);

        values.push(check_digit);

        let mut multiplier = 10;
        let mut sum = 0;
        for value in values {
            sum += value * multiplier;
            multiplier -= 1;
        }

        sum % 11 == 0
    } else {
        false
    }
}
