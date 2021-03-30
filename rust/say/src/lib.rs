pub fn encode(n: u64) -> String {
    digit_to_text(n).to_string()
}

fn digit_to_text<'a>(digit: u64) -> &'a str {
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
        _ => unimplemented!("{} is unimplemented", digit)
    }
}
