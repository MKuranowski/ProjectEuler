use std::collections::HashMap;

use project_euler::is_prime;

#[derive(Debug, Default)]
struct PrimeCache {
    known: HashMap<i32, bool>,
}

impl PrimeCache {
    #[inline]
    fn cached(&self, n: i32) -> Option<bool> {
        self.known.get(&n).copied()
    }

    #[inline]
    fn compute(&mut self, n: i32) -> bool {
        let result: bool = is_prime(n);
        self.known.insert(n, result);
        result
    }

    fn check(&mut self, n: i32) -> bool {
        if let Some(cached) = self.cached(n) {
            cached
        } else {
            self.compute(n)
        }
    }
}

fn len_of_prime_chain_for_quadratic(primes: &mut PrimeCache, a: i32, b: i32) -> i32 {
    let mut n = 0;
    loop {
        let v = (n * n) + (a * n) + b;
        if v > 1 && primes.check(v) {
            n += 1;
        } else {
            break n;
        }
    }
}

fn main() {
    let mut primes = PrimeCache::default();
    let mut max_coefficients_product = 0;
    let mut max_len = 0;

    for a in -999..=999 {
        for b in -999..=999 {
            let len = len_of_prime_chain_for_quadratic(&mut primes, a, b);
            if len > max_len {
                max_len = len;
                max_coefficients_product = a * b;
            }
        }
    }

    println!("{max_coefficients_product}");
}
