pub fn encode(n: u64) -> String {
    //println!("{:?}", split_number_to_thousands(&n));

    split_number_to_thousands(&n)
        .iter()
        .map(|group| number_to_text(group))
        .collect()
}

fn number_to_text(number: &u64) -> String {
    let split = split_thousand_to_smaller(number);

    match split.as_slice() {
        [hundreds, tens, ones] if *hundreds > 0 && *tens > 0 && *ones > 0 => [
            match_ones(&(*hundreds / 100)),
            " ",
            "hundred",
            " ",
            match_tens(tens),
            "-",
            match_ones(ones),
        ]
        .concat(),
        [hundreds, tens, 0] if *hundreds > 0 && *tens > 0 => [
            match_ones(&(*hundreds / 100)),
            " ",
            "hundred",
            " ",
            match_tens(tens),
        ]
        .concat(),
        [hundreds, 0, 0] if *hundreds > 0 => {
            [match_ones(&(*hundreds / 100)), " ", "hundred"].concat()
        }
        [0, tens, ones] if *tens > 0 && *ones > 0 => {
            [match_tens(tens), "-", match_ones(ones)].concat()
        }
        [0, tens, 0] if *tens > 0 => match_tens(tens).to_string(),
        [0, 0, ones] => match ones {
            0..=9 => match_ones(ones),
            10 => match_tens(ones),
            11..=u64::MAX => match_teens(ones),
        }
        .to_string(),
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

fn split_number_to_thousands(number: &u64) -> Vec<u64> {
    let mut remainder = *number;
    let mut split = Vec::new();

    while remainder > 0 || split.is_empty() {
        let part = remainder % 1000;
        split.insert(0, part);
        remainder /= 1000;
    }

    split
}

const SPLIT_POINTS: [u64; 2] = [100, 20];

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
