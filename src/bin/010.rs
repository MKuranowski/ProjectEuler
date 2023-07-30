use project_euler::PrimeSieve;

pub fn main() {
    let result: u64 = PrimeSieve::default().take_while(|&n| n < 2_000_000).sum();
    println!("{result}");
}
