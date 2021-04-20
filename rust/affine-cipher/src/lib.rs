/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    const ALPHABET_COUNT: i32 = 26;
    const ASCII_LOWERCASE_SECTION_START: i32 = 97;
    const GROUP_SIZE: usize = 5;

    Ok(plaintext
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| (c as i32) - ASCII_LOWERCASE_SECTION_START)
        .map(|n| (a * n + b) % ALPHABET_COUNT)
        .map(|n| ((n + ASCII_LOWERCASE_SECTION_START) as u8) as char)
        .enumerate()
        .flat_map(|(i, c)| {
            (i != 0 && i % GROUP_SIZE == 0).then(|| ' ')
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
}
