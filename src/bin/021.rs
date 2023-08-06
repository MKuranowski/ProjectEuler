use std::collections::HashMap;

use num::{integer::Roots, Integer};

fn sum_divisors(n: u32) -> u32 {
    // Divisors come in pairs, so iteration only needs to end at the sqrt.
    // We're supposed to count `1`, but not `n` => iteration starts at 2, and 1 is added explicitly
    1 + (2..=n.sqrt())
        .filter_map(|possible_divisor| {
            let (other, rest) = n.div_rem(&possible_divisor);
            if rest == 0 {
                Some(possible_divisor + other)
            } else {
                None
            }
        })
        .sum::<u32>()
}

#[derive(Debug, Default)]
struct CachedDivisorSumCalculator {
    cached: HashMap<u32, u32>,
}

impl CachedDivisorSumCalculator {
    fn sum_divisors(&mut self, n: u32) -> u32 {
        *self.cached.entry(n).or_insert_with(|| sum_divisors(n))
    }

    fn is_amicable(&mut self, n: u32) -> bool {
        let a = self.sum_divisors(n);
        let b = self.sum_divisors(a);
        a != n && b == n
    }
}

fn main() {
    let mut calculator = CachedDivisorSumCalculator::default();
    let result = (2..=10_000_u32)
        .filter(|&n| calculator.is_amicable(n))
        .sum::<u32>();
    println!("{result}")
}
