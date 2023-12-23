fn is_palindrome(x: &str) -> bool {
    let first_half_end = x.len() / 2;
    let second_half_start = (x.len() + 1) / 2;

    (&x[second_half_start..])
        .bytes()
        .rev()
        .eq((&x[..first_half_end]).bytes())
}

fn main() {
    let result: u32 = (1..1_000_000_u32)
        .filter(|&n| is_palindrome(&n.to_string()) && is_palindrome(&format!("{n:b}")))
        .sum();
    println!("{result}");
}
