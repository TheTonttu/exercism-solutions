pub fn is_armstrong_number(num: u32) -> bool {
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
