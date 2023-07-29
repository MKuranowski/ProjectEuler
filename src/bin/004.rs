// Iterator for palindromes of N*2 digits
struct Palindromes<const N: u32> {
    counter: u32,
}

impl<const N: u32> Palindromes<N> {
    const SHIFT: u32 = 10_u32.pow(N);
    const START: u32 = Self::SHIFT - 1;
    const END: u32 = 10_u32.pow(N - 1);
}

impl<const N: u32> Default for Palindromes<N> {
    fn default() -> Self {
        Self {
            counter: Self::START,
        }
    }
}

impl<const N: u32> Iterator for Palindromes<N> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Ensure we generate a valid palindrome
        if self.counter <= Self::END {
            None
        } else {
            let mut result: u32 = 0;

            // Bottom part
            let mut digits = self.counter;
            for i in 0..N {
                result += digits % 10;
                digits /= 10;
                if i < N - 1 {
                    result *= 10;
                }
            }

            // Upper part
            result += self.counter * Self::SHIFT;

            self.counter -= 1;
            Some(result)
        }
    }
}

fn main() {
    const N: u32 = 3;
    for n in Palindromes::<N>::default() {
        // Iterate over pairs of numbers which (potentially) multiply to n
        for x in (n / Palindromes::<N>::START)..Palindromes::<N>::START {
            let (y, r) = (n / x, n % x);
            if r == 0 {
                println!("{n} = {x} * {y}");
                return;
            }
        }
    }
}
