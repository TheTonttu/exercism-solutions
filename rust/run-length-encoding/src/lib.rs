pub fn encode(source: &str) -> String {
    const START_COUNT: i32 = 1;

    let mut encoded = Vec::new();

    for char in source.chars() {
        match encoded.last_mut() {
            Some((c, count)) if char == *c => {
                *count += 1;
            }
            _ => {
                encoded.push((char, START_COUNT));
            }
        }
    }

    encoded
        .iter()
        .map(|(char, count)| {
            if *count < 2 {
                char.to_string()
            } else {
                [count.to_string(), char.to_string()].concat()
            }
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    const BASE: u32 = 10;

    let mut decoded = Vec::new();

    let mut curr_count = None;
    for char in source.chars() {
        if let Some(digit) = char.to_digit(BASE) {
            match curr_count {
                Some(count) => curr_count = Some(count * BASE + digit),
                None => curr_count = Some(digit),
            }
        } else {
            match curr_count {
                Some(count) => decoded.push((char, count)),
                None => decoded.push((char, 1)),
            }
            curr_count = None;
        }
    }

    decoded
        .iter()
        .map(|(char, count)| char.to_string().repeat(*count as usize))
        .collect()
}
