pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut remaining = n;
    let mut prime_gen = PrimeGenerator::new();

    let mut prime = prime_gen.next().unwrap();
    while prime <= remaining && remaining > 0 {
        while remaining % prime == 0 {
            factors.push(prime);
            remaining /= prime;
        }
        prime = prime_gen.next().unwrap();
    }
    factors
}

struct PrimeGenerator {
    primes: Vec<u64>,
    next: u64
}

impl PrimeGenerator {
    fn new() -> PrimeGenerator {
        PrimeGenerator {
            primes: vec!(2),
            next: 2
        }
    }

    fn is_even(number:&u64) -> bool {
        number % 2 == 0
    }

    fn is_composite_of_previous_primes(prime_candidate: &u64, prev_primes: &Vec<u64>) -> bool {
        let candidate_sqrt = (*prime_candidate as f64).sqrt() as u64;

        for prime in prev_primes {
            if prime > &candidate_sqrt {
                return false;
            }

            if prime_candidate % prime == 0 {
                return true;
            }
        }

        false
    }
}

// Modified bad implementation from nth-prime exercise...
impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let last_prime = *self.primes.last().expect("There should be at least one prime value in the beginning");
        if last_prime == self.next {
            self.next += 1;
            return Some(last_prime);
        }

        let mut prime_candidate= last_prime;
        loop {
            prime_candidate += 1;
            if PrimeGenerator::is_even(&prime_candidate) {
                continue;
            }
            if PrimeGenerator::is_composite_of_previous_primes(&prime_candidate, &self.primes) {
                continue;
            }
            self.primes.push(prime_candidate);
            break;
        }
        Some(prime_candidate)
    }
}

