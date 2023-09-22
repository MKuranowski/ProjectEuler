#[derive(Debug)]
pub struct Digits {
    x: u64,
    base: u8,
}

impl Digits {
    pub fn new<T: Into<u64>>(x: T) -> Self {
        Self {
            x: x.into(),
            base: 10,
        }
    }

    pub fn new_with_base<T: Into<u64>>(x: T, base: u8) -> Self {
        Self { x: x.into(), base }
    }
}

impl Iterator for Digits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > 0 {
            let digit = (self.x % self.base as u64) as u8;
            self.x /= self.base as u64;
            Some(digit)
        } else {
            None
        }
    }
}
