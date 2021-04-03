use itertools::Itertools;

// American names
const SCALE_WORDS: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(n: u64) -> String {
    //println!("{:?}", split_number_to_thousands(&n));

    let thousand_splits = split_number_to_thousands(&n);

    // assign scale index
    let scale_indexed: Vec<(usize, u64)> = thousand_splits
        .iter()
        .rev()
        .enumerate()
        .map(|(index, value)| (index, *value))
        .rev()
        .collect();

    println!("{:?}", scale_indexed);

    scale_indexed
        .iter()
        .filter_map(|(index, value)| {
            println!("{:?}", (index, value));

            // Remove thousand groups with only zero that have values in front.
            match (value, scale_indexed.get(index + 1)) {
                (0, Some(_)) => None,
                (_, None) => Some((index, value)),
                _ => Some((index, value)),
            }
        })
        .map(|(scale_index, value)| {
            [&number_to_text(value), " ", SCALE_WORDS[*scale_index]]
                .concat()
                // Get rid of ending whitespace if scale word is empty
                .trim_end()
                .to_string()
        })
        .join(" ")
}

fn number_to_text(number: &u64) -> String {
    let digits = number_to_digits(number);

    match digits.as_slice() {
        [hundreds, 0, 0] => match_hundreds(hundreds),
        [hundreds, 0, ones] => [&match_hundreds(hundreds), " ", match_ones(ones)].concat(),
        [hundreds, tens, 0] => [&match_hundreds(hundreds), " ", match_tens(tens)].concat(),
        [hundreds, tens, ones] => [
            &match_hundreds(hundreds),
            " ",
            &match_tens_and_ones(tens, ones),
        ]
        .concat(),
        [tens, 0] => match_tens(tens).to_string(),
        [tens, ones] => match_tens_and_ones(tens, ones),
        [ones] => match_ones(ones).to_string(),
        _ => "".to_string(),
    }
}

fn match_ones<'a>(digit: &u64) -> &'a str {
    match digit {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!("{} should not match with 0-9 range", digit),
    }
}

fn match_teens<'a>(digit: &u64) -> &'a str {
    match digit {
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => panic!("{} should not match with 11-19 range", digit),
    }
}

fn match_tens<'a>(digit: &u64) -> &'a str {
    match digit {
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => panic!("{} not matched in tens", digit),
    }
}

fn match_hundreds(digit: &u64) -> String {
    [match_ones(&digit), " hundred"].concat()
}

fn match_tens_and_ones(tens: &u64, ones: &u64) -> String {
    const DIGIT_TEN: u64 = 1;

    match (tens, ones) {
        (0, ones) => match_ones(ones).to_string(),
        (tens, 0) => match_tens(tens).to_string(),
        (tens, ones) if *tens == DIGIT_TEN => match_teens(&(tens * 10 + ones)).to_string(),
        _ => [match_tens(tens), "-", match_ones(ones)].concat(),
    }
}

pub fn split_number_to_thousands(number: &u64) -> Vec<u64> {
    const GROUPING: u64 = 1000;

    let mut remainder = *number;
    let mut split = Vec::new();

    while remainder >= GROUPING {
        let part = remainder % GROUPING;
        split.insert(0, part);
        remainder /= GROUPING;
    }
    let last_group = remainder;
    split.insert(0, last_group);

    split
}

pub fn number_to_digits(number: &u64) -> Vec<u64> {
    const DIGIT_BASE: u64 = 10;

    let mut digits = Vec::new();

    let mut remainder = *number;
    while remainder > 9 {
        let digit = remainder % DIGIT_BASE;
        digits.insert(0, digit);
        remainder /= DIGIT_BASE;
    }
    let last_digit = remainder;
    digits.insert(0, last_digit);

    digits
}
