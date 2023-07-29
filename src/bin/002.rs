struct Fib {
    current: u32,
    next: u32,
}

impl Default for Fib {
    fn default() -> Self {
        Self {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.current, self.next) = (self.next, self.current + self.next);
        Some(self.current)
    }
}

fn main() {
    let result: u32 = Fib::default()
        .take_while(|&n| n <= 4_000_000)
        .filter(|&n| n % 2 == 0)
        .sum();
    println!("{result}")
}
