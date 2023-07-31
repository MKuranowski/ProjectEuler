use num::{BigUint, One};

fn main() {
    let n: BigUint = BigUint::one() << 1000;
    let digits = n.to_string();
    let result = digits.bytes().map(|byte| (byte - b'0') as u64).sum::<u64>();
    println!("{result}");
}
