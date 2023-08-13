use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn find_recurring_cycle_length(denominator: u32) -> u32 {
    if denominator <= 1 {
        return 0;
    }

    let mut seen_dividends: HashMap<u32, u32> = HashMap::default();
    let mut dividend: u32 = 1;
    let mut n: u32 = 0;

    loop {
        dividend = (dividend % denominator) * 10;

        // Long division has terminated - unit fraction has finite decimal expansion
        if dividend == 0 {
            return 0;
        }

        match seen_dividends.entry(dividend) {
            // Cycle in long division detected - return its length
            Entry::Occupied(e) => return n - e.get(),

            // No cycle yet - remember the index of dividend and continue with long division
            Entry::Vacant(e) => e.insert(n),
        };

        n += 1;
    }
}

fn main() {
    let result = (1..1000_u32)
        .map(|d| (d, find_recurring_cycle_length(d)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;

    println!("{result}")
}
