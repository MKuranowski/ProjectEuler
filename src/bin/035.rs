use std::collections::HashSet;

use num::Integer;
use project_euler::PrimeSieve;

#[derive(Debug, Copy, Clone)]
struct Rotations {
    original: u32,
    current: u32,
    insert_pow: u32,
    started: bool,
}

impl Rotations {
    fn new(n: u32) -> Self {
        let insert_pow = 10_u32.pow((n as f32).log10().floor() as u32);

        Self {
            original: n,
            current: n,
            insert_pow,
            started: false,
        }
    }
}

impl Iterator for Rotations {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.current);
        }

        let (base, rot) = self.current.div_rem(&10);
        self.current = rot * self.insert_pow + base;
        if self.current != self.original {
            Some(self.current)
        } else {
            None
        }
    }
}

fn main() {
    let primes: HashSet<u32> = PrimeSieve::default()
        .take_while(|&p| p < 1_000_000)
        .map(|p| p as u32)
        .collect();

    let result = primes
        .iter()
        .copied()
        .filter(|&p| Rotations::new(p).all(|rotation| primes.contains(&rotation)))
        .count();

    println!("{result}");
}
