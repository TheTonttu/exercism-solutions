pub fn encode(n: u64) -> String {
    number_to_text(n).to_string()
}

fn number_to_text<'a>(number: u64) -> &'a str {
    match number {
        0..=9 => match number {
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
        },
        10 => "ten",
        11..=19 => match number {
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
        },
        _ => unimplemented!("{} is unimplemented", number),
    }
}
