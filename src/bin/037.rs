use std::collections::{HashSet, VecDeque};

use project_euler::digits::Digits;
use project_euler::PrimeSieve;

struct Truncations {
    digits: VecDeque<u8>,
    from_least_significant: bool,
}

impl Truncations {
    fn new(x: u64, from_least_significant: bool) -> Self {
        Self {
            digits: Digits::new(x).collect(),
            from_least_significant,
        }
    }

    fn current(&self) -> u64 {
        let mut x: u64 = 0;
        for &digit in self.digits.iter().rev() {
            x = (x * 10) + digit as u64;
        }
        x
    }
}

impl Iterator for Truncations {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.digits.is_empty() {
            return None;
        }

        let result = self.current();
        if self.from_least_significant {
            self.digits.pop_front();
        } else {
            self.digits.pop_back();
        }

        return Some(result);
    }
}

fn main() {
    let primes: HashSet<_> = PrimeSieve::default()
        .take_while(|&p| p < 10_000_000)
        .collect();

    let result = primes
        .iter()
        .copied()
        .filter(|&p| {
            p > 10
                && Truncations::new(p, true).all(|t| primes.contains(&t))
                && Truncations::new(p, false).all(|t| primes.contains(&t))
        })
        .sum::<u64>();

    eprintln!("{result}");
}
