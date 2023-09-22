// The only way to use all 9 digits is to explore all
// 1digit * 4digits = 4digits and 2digits * 3digits = 4digits products.
// 2digits * 2digits gets at most 4 digits (99 * 99 = 9801),
// 3digits * 3digits gets at least 5 digits (100 * 100 = 10000).
//
// The question at hand asks to count each product only once, thus the serach
// will iterate over all 4 digit numbers.

use num::Integer;
use project_euler::{bitset, digits::Digits};

fn has_pan_digital_multiplier(product: u32) -> bool {
    let target = bitset::Small::from([1_usize, 2, 3, 4, 5, 6, 7, 8, 9]);
    let product_digits: bitset::Small = Digits::new(product).collect();

    for a in 1..100_u32 {
        let (b, rest) = product.div_rem(&a);
        if rest != 0 {
            continue;
        }

        let mut multiplier_product_digits = product_digits.clone();
        for digit in Digits::new(a) {
            multiplier_product_digits.insert(digit.into());
        }
        for digit in Digits::new(b) {
            multiplier_product_digits.insert(digit.into());
        }

        if multiplier_product_digits == target {
            return true;
        }
    }
    return false;
}

fn main() {
    let result = (1000..10_000_u32)
        .filter(|&n| has_pan_digital_multiplier(n))
        .sum::<u32>();
    println!("{result}")
}
