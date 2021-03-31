pub fn encode(n: u64) -> String {
    //println!("{:?}", split_number_to_thousands(n));

    number_to_text(n)
}

fn number_to_text<'a>(number: u64) -> String {
    let mut text = String::new();

    let mut reminder = number;
    if reminder >= 20 {
        let part = reminder % 20;

        println!("{}", part);
        let twenty_and_over = reminder - part;
        println!("{}", twenty_and_over);
        let tens_text_part = match twenty_and_over {
            20 => "twenty",
            30 => "thirty",
            40 => "forty",
            50 => "fifty",
            60 => "sixty",
            70 => "seventy",
            80 => "eighty",
            90 => "ninety",
            _ => panic!("{} not matched in >= 20", twenty_and_over),
        };

        text.push_str(tens_text_part);
        reminder -= twenty_and_over;
        println!("{}", reminder);
    }

    let leftover_text_part = match reminder {
        0..=9 => match reminder {
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
            _ => panic!("{} should not match with 0-9 range", reminder),
        },
        10 => "ten",
        11..=19 => match reminder {
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => panic!("{} should not match with 11-19 range", reminder),
        },
        _ => unimplemented!("{} is unimplemented", reminder),
    };
    if text.is_empty() {
        text.push_str(leftover_text_part);
    } else if reminder != 0 {
        text.push('-');
        text.push_str(leftover_text_part);
    }

    text
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
