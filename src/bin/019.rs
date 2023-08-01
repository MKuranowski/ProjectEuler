fn is_leap(y: u32) -> bool {
    if y % 400 == 0 {
        true
    } else if y % 100 == 0 {
        false
    } else if y % 4 == 0 {
        true
    } else {
        false
    }
}

fn days_in_month(m: u32, is_leap: bool) -> u32 {
    // NOTE: Zero-based months

    match m {
        1 => {
            if is_leap {
                29
            } else {
                28
            }
        }
        3 | 5 | 8 | 10 => 30,
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        _ => panic!("invalid month: {m}"),
    }
}

fn main() {
    // Since 1900 starts on a monday and is not leap, 1901 starts on a tuesday.
    // In this program 0 represents monday, 6 sunday; 0 represents January

    let mut result: u32 = 0;

    let mut year: u32 = 1901;
    let mut month: u32 = 0;
    let mut starting_weekday: u32 = 1;

    while year <= 2000 {
        result += (starting_weekday == 6) as u32;

        starting_weekday = (starting_weekday + days_in_month(month, is_leap(year))) % 7;
        month += 1;
        year += month / 12;
        month %= 12;
    }

    println!("{result}");
}
