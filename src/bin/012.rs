use num::{integer::Roots, Integer};

fn count_divisors(n: u64) -> u64 {
    let n_sqrt = n.sqrt();

    // Divisors come in pairs*, so find all the divisors smaller than sqrt and double that amount
    let divisors = (1..=n_sqrt)
        .filter(|possible_divisor| n.is_multiple_of(possible_divisor))
        .count() as u64
        * 2;

    // * unless that number is a square - then sqrt(n) doesn't have a pairing
    if n_sqrt * n_sqrt == n {
        divisors - 1
    } else {
        divisors
    }
}

fn main() {
    let result = (1_u64..)
        .map(|i| i * (i + 1) / 2)
        .skip_while(|&n| count_divisors(n) <= 500)
        .next()
        .unwrap();
    println!("{result}")
}
