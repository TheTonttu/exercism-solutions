const ALPHABET_COUNT: i32 = 26;
const ASCII_LOWERCASE_SECTION_START: i32 = 'a' as i32;

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
        // Add grouping
        .flat_map(|(i, c)| {
            (i != 0 && i % GROUP_SIZE == 0)
                .then(|| ' ')
                .into_iter()
                .chain(std::iter::once(c))
        })
        .collect())
}

fn encode_char(char: &char, a: &i32, b: &i32) -> Option<char> {
    match char {
        alphabetic if alphabetic.is_alphabetic() => {
            let num = get_alphabet_position(alphabetic);
            let encrypted = (a * num + b) % ALPHABET_COUNT;
            let encrypted_char = get_alphabet_from_position(&encrypted);

            Some(encrypted_char)
        }
        numeric if numeric.is_numeric() => Some(*numeric),
        _ => None,
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
    validate_keys(&a, &b)?;

    let a_mmi = mmi(&a, &ALPHABET_COUNT);

    Ok(ciphertext
        .to_ascii_lowercase()
        .chars()
        .filter_map(|c| decode_char(&c, &a_mmi, &b))
        .collect())
}

fn decode_char(char: &char, a_mmi: &i32, b: &i32) -> Option<char> {
    match char {
        alphabetic if alphabetic.is_alphabetic() => {
            let num = get_alphabet_position(alphabetic);

            let decrypted = (a_mmi * (num - b)).rem_euclid(ALPHABET_COUNT);
            let decrypted_char = get_alphabet_from_position(&decrypted);

            Some(decrypted_char)
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

fn get_alphabet_position(alphabetic: &char) -> i32 {
    (*alphabetic as i32) - ASCII_LOWERCASE_SECTION_START
}

fn get_alphabet_from_position(alphabet_position: &i32) -> char {
    ((alphabet_position + ASCII_LOWERCASE_SECTION_START) as u8) as char
}

/// Calculates modular multiplicative inverse `n` of `a`
/// where a*n mod m = 1
fn mmi(a: &i32, m: &i32) -> i32 {
    let mut candidate = 1;
    loop {
        if a * candidate % m == 1 {
            return candidate;
        }
        candidate += 1;
    }
}
