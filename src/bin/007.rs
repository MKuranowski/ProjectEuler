use project_euler::PrimeSieve;

fn main() {
    let result = PrimeSieve::default().nth(10_000).unwrap();
    println!("{result}")
}
