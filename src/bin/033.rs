use num::rational::Ratio;

fn numbers_with_digit(common: u32) -> impl Iterator<Item = (u32, u32)> {
    (1..10_u32).flat_map(move |other| [(other * 10 + common, other), (common * 10 + other, other)])
}

fn main() {
    let one = Ratio::<u32>::new(1, 1);
    let mut result = Ratio::<u32>::new(1, 1);

    for common in 1..10_u32 {
        for (numerator, numer_without_common) in numbers_with_digit(common) {
            for (denominator, denom_without_common) in numbers_with_digit(common) {
                let original = Ratio::new(numerator, denominator);
                let stupidly_simplified = Ratio::new(numer_without_common, denom_without_common);

                if original >= one {
                    continue;
                }

                if original == stupidly_simplified {
                    eprintln!(
                        "{numerator}/{denominator} = {numer_without_common}/{denom_without_common}"
                    );

                    result *= original;
                }
            }
        }
    }

    println!("{}", result.denom());
}
