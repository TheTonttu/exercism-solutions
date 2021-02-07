const FIRST_PRIME:u32 = 2;

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return FIRST_PRIME;
    }

    let primes = calculate_primes(&n);
    primes[(n as usize)]
}

fn calculate_primes(n: &u32) -> Vec<u32> {
    let mut primes:Vec<u32> = Vec::new();
    primes.push(FIRST_PRIME);

    let mut prime_candidate = *primes.last().expect("There should be at least one prime value in the beginning");
    while primes.len() <= (*n as usize) {
        prime_candidate += 1;
        if is_even(&prime_candidate) {
            continue;
        }
        if is_dividable_with_any(&prime_candidate, &primes) {
            continue;
        }
        primes.push(prime_candidate);
    }
    primes
}

fn is_even(number:&u32) -> bool {
    number % 2 == 0
}

fn is_dividable_with_any(number:&u32, dividends:&[u32]) -> bool {
    dividends.iter().any(|dividend|number % dividend == 0)
}