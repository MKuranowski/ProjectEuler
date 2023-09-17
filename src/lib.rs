use num::integer::Roots;
use num::Integer;

pub mod combinatorics;

pub fn is_prime<I: Integer + Copy + From<u8>>(n: I) -> bool {
    if n == 2.into() || n == 3.into() {
        return true;
    }
    if n <= 1.into() || n.is_multiple_of(&2.into()) || n.is_multiple_of(&3.into()) {
        return false;
    }

    let mut i = I::from(5);
    while i * i <= n {
        if n.is_multiple_of(&i) || n.is_multiple_of(&(i + 2.into())) {
            return false;
        }
        i = i + 6.into();
    }
    return true;
}

pub struct PrimeSieve {
    seen: Vec<u64>,
    candidate: u64,
}

impl Default for PrimeSieve {
    fn default() -> Self {
        Self {
            seen: Vec::default(),
            candidate: 2,
        }
    }
}

impl Iterator for PrimeSieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Trivial cases
        if self.candidate == 2 {
            self.seen.push(2);
            self.candidate = 3;
            return Some(2);
        } else if self.candidate == 3 {
            self.seen.push(3);
            self.candidate = 5;
            return Some(3);
        }

        // Sieve, only for n%6 == 1 or n%6 == 5
        let mut result: Option<u64> = None;
        while result.is_none() {
            let divisor_limit = self.candidate.sqrt();
            if self
                .seen
                .iter()
                .copied()
                .take_while(|&prime| prime <= divisor_limit)
                .all(|prime| !self.candidate.is_multiple_of(&prime))
            {
                result = Some(self.candidate);
                self.seen.push(self.candidate);
            }

            // Move to next candidate
            self.candidate += if self.candidate % 6 == 1 { 4 } else { 2 }
        }
        result
    }
}

pub fn sum_proper_divisors(n: u32) -> u32 {
    // Divisors come in pairs, so iteration only needs to end at the sqrt.
    // We're supposed to count `1`, but not `n` => iteration starts at 2, and 1 is added explicitly
    1 + (2..=n.sqrt())
        .filter_map(|possible_divisor| {
            let (other, rest) = n.div_rem(&possible_divisor);
            if rest != 0 {
                None
            } else if possible_divisor != other {
                Some(possible_divisor + other)
            } else {
                Some(possible_divisor)
            }
        })
        .sum::<u32>()
}
