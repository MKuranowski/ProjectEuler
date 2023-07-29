use num::{integer::Roots, Integer};
use project_euler::is_prime;

fn main() {
    let n = 600851475143_u64;
    let mut divisor = n.sqrt();

    // Don't bother checking even divisors
    // NOTE: Could also optimize by filtering out multiples of 3
    if divisor.is_even() {
        divisor -= 1
    }

    while divisor > 1 {
        if n.is_multiple_of(&divisor) && is_prime(divisor) {
            println!("{divisor}");
            break;
        }

        divisor -= 2
    }

    // After reading the writeup I don't think this solution works for every n?
}
