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
    unimplemented!("Return the run-length decoding of {}.", source);
}
