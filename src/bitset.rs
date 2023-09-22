/// Small is a simple bit-set for numbers in range <0, usize::BITS-1>.
/// It is implemented using bitwise operations an a single usize.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Small(usize);

impl Small {
    const MAX_VALUE: usize = usize::BITS as usize - 1;

    pub const fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn clear(&mut self) {
        self.0 = 0
    }

    pub const fn difference(&self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    pub const fn intersection(&self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub const fn union(&self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn contains(&self, value: usize) -> bool {
        if value <= Self::MAX_VALUE {
            self.0 & (1 << value) != 0
        } else {
            false
        }
    }

    pub const fn is_disjoint(&self, other: Self) -> bool {
        self.0 & other.0 == 0
    }

    pub const fn is_subset(&self, other: Self) -> bool {
        self.0 & other.0 == self.0
    }

    pub const fn is_superset(&self, other: Self) -> bool {
        other.0 & self.0 == other.0
    }

    pub fn insert(&mut self, value: usize) {
        assert!(value <= Self::MAX_VALUE);
        self.0 |= 1 << value;
    }

    pub fn remove(&mut self, value: usize) {
        assert!(value <= Self::MAX_VALUE);
        self.0 &= !(1 << value);
    }
}

impl<E: Into<usize>, const N: usize> From<[E; N]> for Small {
    fn from(value: [E; N]) -> Self {
        let mut set = Self(0);
        for item in value {
            set.insert(item.into());
        }
        return set;
    }
}

impl<E: Into<usize>> FromIterator<E> for Small {
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        let mut set = Self(0);
        for item in iter {
            set.insert(item.into());
        }
        return set;
    }
}

impl IntoIterator for Small {
    type IntoIter = SmallIterator;
    type Item = usize;

    fn into_iter(self) -> Self::IntoIter {
        SmallIterator {
            set: self.0,
            n: 0,
            started: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SmallIterator {
    set: usize,
    n: usize,
    started: bool,
}

impl Iterator for SmallIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // Shift the last-generated element, except if not started
        if self.started {
            (self.set, self.n) = (self.set >> 1, self.n + 1);
        } else {
            self.started = true;
        }

        // End of iteration
        if self.set == 0 {
            return None;
        }

        // Calculate the offset to the next number
        let offset = self.set.trailing_zeros() as usize;
        (self.set, self.n) = (self.set >> offset, self.n + offset);
        debug_assert!(self.set & 1 == 1);
        return Some(self.n);
    }
}
