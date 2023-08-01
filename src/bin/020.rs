use num::BigUint;

fn fact(mut n: BigUint) -> BigUint {
    let mut result = BigUint::from(1_u8);

    let two: BigUint = BigUint::from(2_u8);
    while &n >= &two {
        result *= &n;
        n -= 1_u8;
    }

    result
}

fn main() {
    let result_num = fact(BigUint::from(100_u8));
    let result = result_num
        .to_string()
        .as_bytes()
        .iter()
        .map(|n| (n - b'0') as u32)
        .sum::<u32>();
    println!("{result}")
}
