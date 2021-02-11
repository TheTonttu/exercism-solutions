use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_multiples = HashSet::new();
    let mut sum = 0;
    for factor in factors {
        if is_factor_unusable(factor, limit) {
            continue;
        }

        for multiplier in 1.. {
            let multiple = multiplier * factor;
            if multiple >= limit {
                break;
            }
            if !unique_multiples.contains(&multiple) {
                unique_multiples.insert(multiple);
                sum += multiple;
            }
        }
    }

    sum
}

fn is_factor_unusable(factor: &u32, limit: u32) -> bool {
    *factor == 0 || *factor >= limit
}
