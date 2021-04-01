pub fn encode(n: u64) -> String {
    //println!("{:?}", split_number_to_thousands(n));

    number_to_text(n)
}

fn number_to_text(number: u64) -> String {

    let split = split_thousand_to_smaller(number);

    let hundreds_text = match split.get(0).copied() {
        Some(hundreds) if hundreds > 0 => {
            match_ones(hundreds / 100)
        }
        Some(_) => "",
        None => "",
    };

    let tens_text = match split.get(1).copied() {
        Some(tens) if tens > 0 => {
            match_tens(tens)
        }
        Some(_) => "",
        None => "",
    };

    // Ones can also contain 10-19 range
    let ones_text = match split.get(2).copied() {
        Some(teens) if teens > 10 => {
            match_teens(teens)
        }
        Some(ten) if ten == 10 => match_tens(ten),
        Some(ones) => match_ones(ones),
        None => "",
    };


    let mut text = String::new();

    if !hundreds_text.is_empty() {
        text.push_str(hundreds_text);
        text.push_str("hundred") // TODO: Plural
    }

    if !tens_text.is_empty() {
        if !text.is_empty() {
            text.push(' ');
        }
        text.push_str(tens_text);
    }

    if !ones_text.is_empty() {
        if !tens_text.is_empty() {
            text.push('-')
        }
        text.push_str(ones_text);
    }

    text
}

fn match_ones<'a>(number: u64) -> &'a str {
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

fn match_teens<'a>(number: u64) -> &'a str {
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

fn match_tens<'a>(number: u64) -> &'a str {
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

fn split_number_to_thousands(number: u64) -> Vec<u64> {
    let mut reminder = number;
    let mut split = Vec::new();

    while reminder > 0 {
        let part = reminder % 1000;
        split.insert(0, part);
        reminder /= 1000;
    }

    split
}

const SPLIT_POINTS: [u64; 2] = [100, 20];

pub fn split_thousand_to_smaller(number: u64) -> Vec<u64> {
    let mut split = Vec::new();

    let mut reminder = number;
    for point in SPLIT_POINTS.iter() {
        let mut part = 0;
        while reminder >= *point {
            part += point;
            reminder -= point;
        }
        split.push(part);
    }
    split.push(reminder);

    split
}
