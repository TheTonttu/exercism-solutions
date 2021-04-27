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
        let mut number_octets = Vec::new();
        let mut remainder = *number;
        while remainder > 0 {
            let extracted_octet = if number_octets.is_empty() {
                (remainder & EXTRACT_BITMASK) as u8
            } else {
                ((remainder & EXTRACT_BITMASK) | SIGN_BIT as u32) as u8
            };

            number_octets.insert(0, extracted_octet);
            remainder >>= OCTET_SIZE;
        }

        if number_octets.is_empty() {
            encoded.push(0);
        } else {
            encoded.extend(number_octets);
        }
    }

    encoded
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut decoded = Vec::new();

    let mut decoded_number = 0u32;
    for octet in bytes {
        decoded_number = add_octet(&decoded_number, octet)?;

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

fn add_octet(number: &u32, octet: &u8) -> Result<u32, Error> {
    if can_fit_octet(&number) {
        let mut result = *number;
        result <<= OCTET_SIZE;
        result |= exclude_sign_bit(octet) as u32;

        Ok(result)
    } else {
        Err(Error::Overflow)
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
