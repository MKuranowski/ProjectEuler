use num::BigUint;
use std::collections::HashSet;

fn main() {
    let mut seen: HashSet<BigUint> = HashSet::default();

    for a in 2..=100_u32 {
        for b in 2..=100_u32 {
            let n = BigUint::from(a).pow(b);
            seen.insert(n);
        }
    }

    println!("{}", seen.len());
}
