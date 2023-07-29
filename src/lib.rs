use num::Integer;

pub fn is_prime<I: Integer + Copy + From<u8>>(n: I) -> bool {
    if n == 2.into() || n == 3.into() {
        return true;
    }
    if n <= 1.into() || n.is_multiple_of(&2.into()) || n.is_multiple_of(&3.into()) {
        return false;
    }

    let mut i = I::from(5);
    while i * i <= n {
        if n.is_multiple_of(&i) || n.is_multiple_of(&(i + 2.into())) {
            return false;
        }
        i = i + 6.into();
    }
    return true;
}
