use std::collections::HashMap;

use project_euler::sum_proper_divisors;

#[derive(Debug, Default)]
struct CachedDivisorSumCalculator {
    cached: HashMap<u32, u32>,
}

impl CachedDivisorSumCalculator {
    fn sum_divisors(&mut self, n: u32) -> u32 {
        *self
            .cached
            .entry(n)
            .or_insert_with(|| sum_proper_divisors(n))
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
