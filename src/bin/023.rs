use std::collections::HashSet;

use project_euler::sum_proper_divisors;

const LIMIT: u32 = 28123;

fn is_abundant(n: u32) -> bool {
    sum_proper_divisors(n) > n
}

struct AbundantCache {
    abundant_numbers: HashSet<u32>,
}

impl AbundantCache {
    fn new() -> Self {
        let abundant_numbers = (12..=LIMIT).filter(|&n| is_abundant(n)).collect();
        Self { abundant_numbers }
    }

    fn is_abundant(&self, n: u32) -> bool {
        self.abundant_numbers.contains(&n)
    }

    fn can_be_written_as_sum_of_two_abundant(&self, n: u32) -> bool {
        // Smallest abundant number is 12, anything below is not worth checking
        for a in 12..=(n / 2) {
            let b = n - a;
            if self.is_abundant(a) && self.is_abundant(b) {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let cache = AbundantCache::new();
    let result = (1..=LIMIT)
        .filter(|&n| !cache.can_be_written_as_sum_of_two_abundant(n))
        .sum::<u32>();
    println!("{result}")
}
