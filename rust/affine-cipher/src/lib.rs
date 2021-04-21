const ALPHABET_COUNT: i32 = 26;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    const GROUP_SIZE: usize = 5;

    validate_keys(&a, &b)?;

    Ok(plaintext
        .to_ascii_lowercase()
        .chars()
        .filter_map(|c| encode_char(&c, &a, &b))
        .enumerate()
        .flat_map(|(i, c)| {
            (i != 0 && i % GROUP_SIZE == 0)
                .then(|| ' ')
                .into_iter()
                .chain(std::iter::once(c))
        })
        .collect())
}

fn encode_char(char: &char, a: &i32, b: &i32) -> Option<char> {
    const ASCII_LOWERCASE_SECTION_START: i32 = 97;

    match char {
        alphabetic if alphabetic.is_alphabetic() => {
            let num = (*alphabetic as i32) - ASCII_LOWERCASE_SECTION_START;
            let encrypted = (a * num + b) % ALPHABET_COUNT;
            let encrypted_char = ((encrypted + ASCII_LOWERCASE_SECTION_START) as u8) as char;

            Some(encrypted_char)
        }
        numeric if numeric.is_numeric() => Some(*numeric),
        _ => None,
    }
}

fn validate_keys(a: &i32, b: &i32) -> Result<(), AffineCipherError> {
    match (a, b) {
        (0, _) => Err(AffineCipherError::NotCoprime(*a)),
        (_, 0) => Err(AffineCipherError::NotCoprime(*b)),
        (a, _) if !are_coprimes(a, &ALPHABET_COUNT) => Err(AffineCipherError::NotCoprime(*a)),
        (a, b) if are_coprimes(a, b) => Ok(()),
        (_, _) => Ok(()),
    }
}

fn are_coprimes(first: &i32, second: &i32) -> bool {
    gcd(first, second) == 1
}

/// Calculates greatest common divisor between numbers `first` and `second`.
fn gcd(first: &i32, second: &i32) -> i32 {
    let mut max = *first;
    let mut min = *second;
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let remainder = max % min;
        if remainder == 0 {
            return min;
        }

        max = min;
        min = remainder;
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
}
