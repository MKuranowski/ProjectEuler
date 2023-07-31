use num::Integer;

struct Collatz {
    i: u64,
    started: bool,
}

impl Collatz {
    fn new(i: u64) -> Self {
        Self { i, started: false }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            Some(self.i)
        } else if self.i == 0 || self.i == 1 {
            None
        } else {
            self.i = if self.i.is_even() {
                self.i / 2
            } else {
                self.i * 3 + 1
            };
            Some(self.i)
        }
    }
}

fn main() {
    let result = (1..1_000_000)
        .map(|i| (i, Collatz::new(i).count()))
        .max_by(|(_, len1), (_, len2)| len1.cmp(len2))
        .unwrap()
        .0;
    println!("{result}");
}
