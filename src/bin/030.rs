fn sum_of_digit_powers(mut n: u32, powers: &[u32; 10]) -> u32 {
    let mut sum: u32 = 0;
    while n > 0 {
        let digit = n % 10;
        n = n / 10;
        sum += powers[digit as usize];
    }
    sum
}

fn main() {
    const POW: u32 = 5;
    let powers: [u32; 10] = std::array::from_fn(|i| i.pow(POW) as u32);

    let result = (10..10_u32.pow(POW + 1))
        .filter(|&n| n == sum_of_digit_powers(n, &powers))
        .sum::<u32>();

    println!("{result}");
}
