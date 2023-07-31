fn main() {
    // This is a simple combinatorics question: on a n by n grid, we have to make
    // n right and n down moves, for a total of 2n moves; thus there are `2n choose n` paths.
    // (2n choose n) = (2n)! / ( (n)! * (2n-n)! ) = (2n)! / (n!)²
    //               = (2n) * (2n-1) * ... * (n+1) / n * (n - 1) * ... * 1

    let mut result = 1.0;
    for i in 1..=20 {
        result *= (20.0 + i as f64) / i as f64
    }
    result = result.round();
    println!("{result}")
}
