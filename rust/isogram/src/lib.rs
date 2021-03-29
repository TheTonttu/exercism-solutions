use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut check_set = HashSet::new();

    candidate
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase())
        .all(|lc| check_set.insert(lc.fold(0, |unicode_sum, char| (unicode_sum + char as u32))))

    // Alternative with lowercase string:
    // candidate
    //     .to_lowercase()
    //     .chars()
    //     .filter(|c| c.is_alphanumeric())
    //     .all(|c| check_set.insert(c))
}
