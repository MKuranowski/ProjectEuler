use project_euler::permutations::Permutator;

fn main() {
    let mut permutator = Permutator::of(10);
    for _ in 0..1_000_000 {
        permutator.next();
    }
    println!("{:?}", permutator.indices());
}
