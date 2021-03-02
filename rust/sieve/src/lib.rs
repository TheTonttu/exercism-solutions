const FIRST_PRIME: usize = 2;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let max = upper_bound as usize + 1;

    let mut sieve_mask = vec![true; max];

    for candidate in FIRST_PRIME..max {
        if !sieve_mask[candidate] {
            continue;
        }

        let composites = sieve_mask
            .iter_mut()
            .skip(candidate * candidate)
            .step_by(candidate);

        for element in composites {
            *element = false;
        }
    }

    sieve_mask
        .iter()
        .enumerate()
        // Skip 0 and 1
        .skip(2)
        .filter_map(|(n, is_prime)| is_prime.then(|| n as u64))
        .collect()
}
