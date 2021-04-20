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
    const ALPHABET_COUNT: i32 = 26;
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

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
}
