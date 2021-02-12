pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut remaining = n;
    let mut prime_gen = Primes::new();

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

struct Primes {
    primes: Vec<u64>,
    next: u64
}

impl Primes {
    fn new() -> Primes {
        Primes {
            primes: vec!(2),
            next: 2
        }
    }

    fn is_even(number:&u64) -> bool {
        number % 2 == 0
    }

    fn is_dividable_with_any(dividend:&u64, divisors:&[u64]) -> bool {
        divisors.iter().any(|divisor| dividend % divisor == 0)
    }
}

// Modified bad implementation from nth-prime exercise...
impl Iterator for Primes {
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
            if Primes::is_even(&prime_candidate) {
                continue;
            }
            if Primes::is_dividable_with_any(&prime_candidate, &self.primes) {
                continue;
            }
            self.primes.push(prime_candidate);
            break;
        }
        Some(prime_candidate)
    }
}

