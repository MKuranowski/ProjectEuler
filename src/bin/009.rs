use num::{integer::Roots, Integer};

fn main() {
    // The solution must lay on a curve `a + b + sqrt(a² + b²) = 1000`
    // Changing to function:
    //           1000 = a + b + sqrt(a² + b²)
    //  sqrt(a² + b²) = 1000 - a - b
    //        a² + b² = (1000 - a - b)²
    //        a² + b² = 1_000_000 + a² + b² - 2000a - 2000b + 2ab
    //              0 = 1_000_000 - 2000a - 2000b + 2ab
    //              0 = 500_000 - 1000a + b(a - 1000)
    //    b(a - 1000) = 1000a - 500_000
    //              b = (1000a - 500_000) / (a - 1000)
    //
    // Trivial solutions are (0, 500) and (500, 0).
    // a and b must be positive and integers, thus confining the search to a, b in <1, 499>
    // Note also that the curve is symmetric along a = b axis, further confining the
    // search to <1, 250>.

    for a in 1..=250_i64 {
        let numerator = 1000 * a - 500_000;
        let denominator = a - 1000;
        let (b, rest) = numerator.div_rem(&denominator);

        // b has to be an integer
        if rest != 0 {
            continue;
        }

        let c = (a * a + b * b).sqrt();
        println!("{}", a * b * c);
    }
}
