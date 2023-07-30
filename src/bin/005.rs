use num::Integer;

fn main() {
    let mut result = 1_u64;
    for n in 2..=20_u64 {
        result = result.lcm(&n);
    }
    println!("{result}")
}
