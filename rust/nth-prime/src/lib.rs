const FIRST_PRIME:u32 = 2;

pub fn nth(n: u32) -> u32 {
    let mut primes:Vec<u32> = Vec::new();
    primes.push(FIRST_PRIME);

    let mut prime_candidate = *primes.last().expect("There should be at least one prime value in the beginning");
    while primes.len() <= (n as usize) {
        prime_candidate += 1;
        if prime_candidate % 2 == 0 {
            continue;
        }
        if primes.iter().any(|prime|prime_candidate % prime == 0) {
            continue;
        }
        primes.push(prime_candidate);
    }
    primes[n as usize]
}