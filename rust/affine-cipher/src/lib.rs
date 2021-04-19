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

    Ok(plaintext
        .to_ascii_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| (c as i32) - ASCII_LOWERCASE_SECTION_START)
        .map(|n| (a * n + b) % ALPHABET_COUNT)
        .map(|n| ((n + ASCII_LOWERCASE_SECTION_START) as u8) as char)
        .collect())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
}
