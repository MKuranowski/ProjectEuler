fn main() {
    // Knowing that Σn = n(n+1) / 2 and Σn² = n(n+1)(2n+1) / 6 one can derive that
    // (Σn)² - Σn² = (3n⁴ + 2n³ - 3n² - 2n) / 12
    const N: u64 = 100;
    const N2: u64 = N * N;
    const N3: u64 = N2 * N;
    const N4: u64 = N3 * N;
    let result = (3 * N4 + 2 * N3 - 3 * N2 - 2 * N) / 12;
    println!("{result}")
}
