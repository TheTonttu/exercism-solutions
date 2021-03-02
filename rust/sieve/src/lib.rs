pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut prime_candidates = (2..=upper_bound).collect::<Vec<_>>();

    let mut primes = Vec::new();
    while !prime_candidates.is_empty() {
        let prime = prime_candidates.remove(0);
        primes.push(prime);
        let composites = (prime*prime..=upper_bound).step_by(prime as usize).collect::<Vec<_>>();
        prime_candidates.retain(|c| !composites.contains(c));
    }

    primes
}
