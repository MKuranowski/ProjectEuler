use project_euler::digits::Digits;

fn sum_of_digits_factorial(n: u32) -> u32 {
    const FACTORIALS: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    Digits::new(n).map(|digit| FACTORIALS[digit as usize]).sum()
}

fn main() {
    let result: u32 = (10..100_000_u32)
        .filter(|&n| n == sum_of_digits_factorial(n))
        .sum();
    println!("{result}")
}
