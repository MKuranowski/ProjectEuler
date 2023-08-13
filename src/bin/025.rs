use num::BigUint;

fn main() {
    let mut n: usize = 1;
    let mut current = BigUint::from(1_u32);
    let mut previous = BigUint::from(0_u32);
    let mut next: BigUint;

    while current.to_string().len() < 1000 {
        next = &current + &previous;
        previous = current;
        current = next;
        n += 1;
    }

    println!("{n}");
}
