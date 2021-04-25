#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const BITMASK_7: u32 = 0b01111111;
const SIGN_BIT: u32 = 0b10000000;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    println!("values hex: {:02X?}", values);

    let mut encoded = Vec::new();

    for number in values {
        let mut extracted = Vec::new();
        let mut remainder = *number;
        while remainder > 0 {
            let extracted_value = if extracted.is_empty() {
                (remainder & BITMASK_7) as u8
            }  else {
                ((remainder & BITMASK_7) | SIGN_BIT) as u8
            };

            println!("extract: {:02X?}", extracted_value);
            extracted.insert(0, extracted_value);
            remainder >>= 7;
        }
        if extracted.is_empty() {
            extracted.push(0);
        }

        encoded.extend(extracted);
    }

    encoded
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
}
