#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    const MIN_BASE: u32 = 2;

    if from_base < MIN_BASE {
        return Err(Error::InvalidInputBase);
    }

    if to_base < MIN_BASE {
        return Err(Error::InvalidOutputBase);
    }

    if let Some(invalid_digit) = number.iter().copied().find(|n| *n >= from_base) {
        return Err(Error::InvalidDigit(invalid_digit));
    }

    let exponents = 0..(number.len() as u32);

    let decimal_number: u32 = number
        .iter()
        .rev()
        .zip(exponents)
        .map(|(digit, exponent)| *digit * from_base.pow(exponent))
        .sum();

    let output_digits = convert_decimal_to_n_base(decimal_number, to_base);

    Ok(output_digits)
}

fn convert_decimal_to_n_base(decimal_number: u32, to_base: u32) -> Vec<u32> {
    let mut remainder = decimal_number;
    let mut output_digits = Vec::new();
    while remainder > 0 {
        let digit = remainder % to_base;
        output_digits.insert(0, digit);
        remainder /= to_base;
    }

    if output_digits.is_empty() {
        output_digits.push(0);
    }
    output_digits
}
