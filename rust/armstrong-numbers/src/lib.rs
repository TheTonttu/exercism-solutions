// u32::MAX digit count
const MAX_CAPACITY_DIGITS_U32: usize = 10;

pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::with_capacity(MAX_CAPACITY_DIGITS_U32);
    let mut remainder = num;

    while remainder > 0 {
        let digit = remainder % 10;
        digits.push(digit);
        remainder /= 10;
    }

    let digit_count = digits.len() as u32;
    num == digits.iter().map(|d| d.pow(digit_count)).sum()
}

// alternative implementation without vector
pub fn is_armstrong_number_without_vector(num: u32) -> bool {
    let digit_count = count_digits(num);
    let mut remainder = num;
    let mut sum = 0;

    while remainder > 0 {
        let digit = remainder % 10;
        sum += digit.pow(digit_count);
        remainder /= 10;
    }

    num == sum
}

fn count_digits(mut num: u32) -> u32 {
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}
