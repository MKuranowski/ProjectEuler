fn main() {
    // The top-right radius is a sequence of odd squares. Going counter-clockwise,
    // numbers on the previous radiuses are spread out by a constant, depending on how far
    // away are we from the number 1.
    //
    // | Radius \ n   | 0 | 1 | 2  | 3  | ... | n            |
    // |--------------|---|---|----|----|-----|--------------|
    // | Top-Right    | 1 | 9 | 25 | 49 |     | (2n+1)²      |
    // | Top-Left     | - | 7 | 21 | 43 |     | (2n+1)² - 2n |
    // | Bottom-Left  | - | 5 | 17 | 37 |     | (2n+1)² - 4n |
    // | Bottom-Right | - | 3 | 13 | 31 |     | (2n+1)² - 6n |
    // |--------------|---|---|----|----|-----|--------------|
    // | Step         | - | 2 | 4  | 6  |     | 2n           |
    //
    // On an x by x grid we have to stop at n = ⌊x/2⌋
    // Thus, the formula is (with summation from i=1 to n):
    // Σ(4(2i+1)² - 12i) + 1 = Σ(16i² + 4i + 4) + 1 = 16Σi² + 4Σi + Σ4 + 1 =
    // = 16(n³/3 + n²/2 + n/6) + 4(n(n+1)/2) + 4n + 1 = 16n³/3 + 8n² + 16n/6 + 2n² + 2n + 4n + 1 =
    // = 16n³/3 + 10n² + 26n/3 + 1 = (16n³ + 30n² + 26n + 3) / 3
    let x = 1001_u32;
    let n = x / 2;
    let n2 = n * n;
    let n3 = n2 * n;
    let result = ((16 * n3) + (30 * n2) + (26 * n) + 3) / 3;
    println!("{result}");
}
