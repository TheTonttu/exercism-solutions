pub fn encode(source: &str) -> String {
    let mut counts = Vec::new();

    let mut curr_char = None;
    let mut count = 0;
    for char in source.chars() {
        if let Some(c) = curr_char {
            if char == c {
                count += 1;
            } else {
                counts.push((c, count));
                curr_char = Some(char);
                count = 1;
            }
        } else {
            curr_char = Some(char);
            count = 1;
        }
    }

    if let Some(c) = curr_char {
        counts.push((c, count));
    }

    counts
        .iter()
        .map(|(char, count)| {
            if *count < 2 {
                char.to_string()
            } else {
                format!("{}{}", count, char)
            }
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    let mut decoding = Vec::new();

    let mut count = None;
    for char in source.chars() {
        match char {
            '0'..='9' => {
                let digit = char.to_digit(10).unwrap() as i32;
                if let Some(n) = count {
                    count = Some(n * 10 + digit);
                } else {
                    count = Some(digit);
                }
            }
            _ => {
                if let Some(n) = count {
                    decoding.push((char, n));
                } else {
                    decoding.push((char, 1));
                }
                count = None;
            }
        }
    }

    decoding
        .iter()
        .map(|(char, count)| char.to_string().repeat(*count as usize))
        .collect()
}
