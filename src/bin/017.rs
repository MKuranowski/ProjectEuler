fn letter_count(n: u32) -> u32 {
    assert!(n <= 9999);
    match n {
        // Base cases for recursion
        0 => "".len() as u32,
        1 => "one".len() as u32,
        2 => "two".len() as u32,
        3 => "three".len() as u32,
        4 => "four".len() as u32,
        5 => "five".len() as u32,
        6 => "six".len() as u32,
        7 => "seven".len() as u32,
        8 => "eight".len() as u32,
        9 => "nine".len() as u32,
        10 => "ten".len() as u32,
        11 => "eleven".len() as u32,
        12 => "twelve".len() as u32,
        13 => "thirteen".len() as u32,
        14 => "fourteen".len() as u32,
        15 => "fifteen".len() as u32,
        16 => "sixteen".len() as u32,
        17 => "seventeen".len() as u32,
        18 => "eighteen".len() as u32,
        19 => "nineteen".len() as u32,
        20 => "twenty".len() as u32,
        30 => "thirty".len() as u32,
        40 => "forty".len() as u32,
        50 => "fifty".len() as u32,
        60 => "sixty".len() as u32,
        70 => "seventy".len() as u32,
        80 => "eighty".len() as u32,
        90 => "ninety".len() as u32,
        _ => {
            if n < 100 {
                // "YYYty XXX"
                letter_count(n / 10 * 10) + letter_count(n % 10)
            } else if n < 1000 {
                // "ZZZ hundred" or "ZZZ hundred and YYY"
                let tens_and_units = letter_count(n % 100);
                let hundred_and = if tens_and_units != 0 {
                    "hundredand".len() as u32
                } else {
                    "hundred".len() as u32
                };
                let hundreds = letter_count(n / 100);
                hundreds + hundred_and + tens_and_units
            } else {
                // "AAA thousand ZZZ"
                let hundreds = letter_count(n % 1000);
                let thousand = "thousand".len() as u32;
                let thousands = letter_count(n / 1000);
                thousands + thousand + hundreds
            }
        }
    }
}

fn main() {
    let result = (1..=1000).map(|i| letter_count(i)).sum::<u32>();
    println!("{result}")
}
