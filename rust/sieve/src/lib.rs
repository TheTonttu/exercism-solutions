const FIRST_PRIME: usize = 2;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let max = upper_bound as usize + 1;

    let mut sieve_mask = vec![true; max];

    let sqrt_max = (max as f64).sqrt() as usize;
    for candidate in FIRST_PRIME..=sqrt_max {
        if !sieve_mask[candidate] {
            continue;
        }

        let prime = candidate;

        let composites = sieve_mask.iter_mut().skip(prime * prime).step_by(prime);
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
