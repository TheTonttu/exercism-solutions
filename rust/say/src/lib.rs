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

    let number_texts: Vec<String> = scale_indexed
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
            if SCALE_WORDS[*scale_index].is_empty() {
                number_to_text(value)
            } else {
                [
                    number_to_text(value),
                    " ".to_string(),
                    SCALE_WORDS[*scale_index].to_string(),
                ]
                .concat()
            }
        })
        .collect();

    number_texts.join(" ")
}

fn number_to_text(number: &u64) -> String {
    let split = split_thousand_to_smaller(number);

    match split.as_slice() {
        [hundreds, tens, ones] if *hundreds > 0 && *tens > 0 && *ones > 0 => [
            &match_hundreds(hundreds),
            " ",
            &match_tens_and_ones(tens, ones),
        ]
        .concat(),
        [hundreds, 0, ones] if *hundreds > 0 && *ones > 0 => {
            [&match_hundreds(hundreds), " ", match_ones(ones)].concat()
        }
        [hundreds, tens, 0] if *hundreds > 0 && *tens > 0 => {
            [&match_hundreds(hundreds), " ", match_tens(tens)].concat()
        }
        [hundreds, 0, 0] if *hundreds > 0 => match_hundreds(hundreds),
        [0, tens, ones] if *tens > 0 && *ones > 0 => match_tens_and_ones(tens, ones),
        [0, tens, 0] if *tens > 0 => match_tens(tens).to_string(),
        [0, 0, ones] => match_ones(ones).to_string(),
        _ => "zero".to_string(),
    }
}

fn match_ones<'a>(number: &u64) -> &'a str {
    match number {
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
        _ => panic!("{} should not match with 0-9 range", number),
    }
}

fn match_teens<'a>(number: &u64) -> &'a str {
    match number {
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => panic!("{} should not match with 11-19 range", number),
    }
}

fn match_tens<'a>(number: &u64) -> &'a str {
    match number {
        10 => "ten",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => panic!("{} not matched in tens", number),
    }
}

fn match_hundreds<'a>(number: &u64) -> String {
    let digit = number / 100;

    match digit {
        0 => "".to_string(),
        _ => [match_ones(&digit), " hundred"].concat(),
    }
}

fn match_tens_and_ones(tens: &u64, ones: &u64) -> String {
    match (tens, ones) {
        (0, ones) => match_ones(ones).to_string(),
        (tens, 0) => match_tens(tens).to_string(),
        (tens, ones) if *tens == 10 => match_teens(&(tens + ones)).to_string(),
        _ => [match_tens(tens), "-", match_ones(ones)].concat(),
    }
}

pub fn split_number_to_thousands(number: &u64) -> Vec<u64> {
    let mut remainder = *number;
    let mut split = Vec::new();

    while remainder > 0 || split.is_empty() {
        let part = remainder % 1000;
        split.insert(0, part);
        remainder /= 1000;
    }

    split
}

const SPLIT_POINTS: [u64; 2] = [100, 10];

pub fn split_thousand_to_smaller(number: &u64) -> Vec<u64> {
    let mut split = Vec::new();

    let mut remainder = *number;
    for point in SPLIT_POINTS.iter() {
        let mut part = 0;
        while remainder >= *point {
            part += point;
            remainder -= point;
        }
        split.push(part);
    }
    split.push(remainder);

    split
}
