pub mod alt_implementations;

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut remaining = n;
    let mut factor_candidate = 2;

    while factor_candidate <= remaining && remaining > 0 {
        while remaining % factor_candidate == 0 {
            factors.push(factor_candidate);
            remaining /= factor_candidate;
        }

        // Check only odd numbers after 2
        factor_candidate += (factor_candidate % 2) + 1
    }
    factors
}
