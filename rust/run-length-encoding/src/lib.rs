pub fn encode(source: &str) -> String {
    let mut counts = Vec::new();

    let mut curr_char = None;
    let mut count = 0;
    for char in source.chars() {
        match curr_char {
            Some(c) if char == c => {
                count += 1;
            }
            Some(c) => {
                counts.push((c, count));
                curr_char = Some(char);
                count = 1;
            }
            None => {
                curr_char = Some(char);
                count = 1;
            }
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
    const BASE: u32 = 10;

    let mut decoding = Vec::new();

    let mut curr_count = None;
    for char in source.chars() {
        if let Some(digit) = char.to_digit(BASE) {
            match curr_count {
                Some(count) => curr_count = Some(count * BASE + digit),
                None => curr_count = Some(digit),
            }
        } else {
            match curr_count {
                Some(n) => decoding.push((char, n)),
                None => decoding.push((char, 1)),
            }
            curr_count = None;
        }
    }

    decoding
        .iter()
        .map(|(char, count)| char.to_string().repeat(*count as usize))
        .collect()
}
