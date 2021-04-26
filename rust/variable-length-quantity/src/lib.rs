#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const EXTRACT_BITMASK: u32 = 0b01111111;
const OCTET_SIZE: u32 = 7;
const SIGN_BIT: u8 = 0b10000000;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    println!("values hex: {:02X?}", values);

    let mut encoded = Vec::new();

    for number in values {
        let mut extracted = Vec::new();
        let mut remainder = *number;
        while remainder > 0 {
            let extracted_value = if extracted.is_empty() {
                (remainder & EXTRACT_BITMASK) as u8
            } else {
                ((remainder & EXTRACT_BITMASK) | SIGN_BIT as u32) as u8
            };

            println!("extract: {:02X?}", extracted_value);
            extracted.insert(0, extracted_value);
            remainder >>= OCTET_SIZE;
        }

        if extracted.is_empty() {
            encoded.push(0);
        } else {
            encoded.extend(extracted);
        }
    }

    encoded
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut decoded = Vec::new();

    let mut decoded_number = 0u32;
    for octet in bytes {
        if !can_fit_octet(&decoded_number) {
            return Err(Error::Overflow);
        }

        decoded_number <<= OCTET_SIZE;
        // Remove possible sign bit
        let extracted = exclude_sign_bit(octet);
        decoded_number |= extracted as u32;

        if is_end_octet(octet) {
            decoded.push(decoded_number);
            decoded_number = 0;
        }
    }

    if decoded.is_empty() {
        Err(Error::IncompleteNumber)
    } else {
        Ok(decoded)
    }
}

fn exclude_sign_bit(octet: &u8) -> u8 {
    octet & !SIGN_BIT
}

fn can_fit_octet(number: &u32) -> bool {
    number.leading_zeros() >= OCTET_SIZE
}

fn is_end_octet(octet: &u8) -> bool {
    octet & SIGN_BIT == 0
}
