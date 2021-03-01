
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-'][..])
        .map(|s| {
            s.trim_matches('_')
                .chars()
                .next()
                .map(|c| c.to_uppercase().collect::<String>())
        })
        .flatten()
        .collect()
}
