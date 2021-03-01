use std::iter::once;

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-'][..])
        // Yikes forever...
        .flat_map(|s|
            // Get first character + uppercase characters from all lowercase-uppercase patterns.
            // Prepend window with None to check separately start of split word.
            once(None)
                .chain(s.trim_matches('_').chars().map(Some))
                .collect::<Vec<_>>()
                .windows(2)
                .filter(|window| match window {
                    [None, Some(_first)] => true,
                    [Some(c1), Some(c2)] => c1.is_lowercase() && c2.is_uppercase(),
                    &_ => false,
                })
                .map(|window| match window {
                    [_, Some(c2)] => c2.to_uppercase().to_string(),
                    &_ => "".to_string()
                })
                .collect::<Vec<_>>())
        .collect()
}
