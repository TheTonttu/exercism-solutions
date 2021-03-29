use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut check_set = HashSet::new();

    candidate
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase())
        .map(|lc| lc.fold(0, |unicode_sum, char| (unicode_sum + char as u32)))
        .all(|uc_sum| check_set.insert(uc_sum))

    // Alternative with lowercase string:
    // candidate
    //     .to_lowercase()
    //     .chars()
    //     .filter(|c| c.is_alphanumeric())
    //     .all(|c| check_set.insert(c))
}
