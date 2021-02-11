use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_multiples = HashSet::new();
    let mut sum = 0;
    for factor in factors {
        for multiplier in 1.. {
            let result = factor * multiplier;
            if result == 0 || result >= limit {
                break;
            }
            if !unique_multiples.contains(&result) {
                unique_multiples.insert(result);
                sum += result;
            }
        }
    }

    sum
}
