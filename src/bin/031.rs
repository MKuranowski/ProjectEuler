use std::collections::HashMap;

#[derive(Debug, Default)]
struct Search {
    cached: HashMap<(usize, u32), u64>,
}

impl Search {
    fn ways_to_build<'a>(&'a mut self, coins: &[u32], amount: u32) -> u64 {
        // Base recursive case
        if amount == 0 {
            return 1;
        }

        // Check if result is cached
        if let Some(cached) = self.cached.get(&(coins.len(), amount)) {
            return *cached;
        }

        // Calculate the ways to build recursively
        let ways: u64 = coins
            .iter()
            .enumerate()
            .map(|(i, &coin)| {
                if coin <= amount {
                    self.ways_to_build(&coins[i..], amount - coin)
                } else {
                    0
                }
            })
            .sum();

        self.cached.insert((coins.len(), amount), ways);
        return ways;
    }
}

fn main() {
    let coins = [200, 100, 50, 20, 10, 5, 2, 1];
    let mut search = Search::default();
    println!("{}", search.ways_to_build(&coins, 200));
}
