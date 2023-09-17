/// CombinatoricIterator is a common trait for iterators over some combinatoric operation
/// over a sequence. CombinatoricIterators work purely on indices into a sequence,
/// that is they always mutate an internal vector of 0..item_count.
pub trait CombinatoricIterator {
    /// of_with_length creates a new CombinatoricIterator over 0..item_count
    /// which generates sequences of item_count items.
    #[inline]
    fn of(item_count: usize) -> Self
    where
        Self: Sized,
    {
        Self::of_with_length(item_count, item_count)
    }

    /// of_with_length creates a new CombinatoricIterator over 0..item_count
    /// which generates sequences of result_len items.
    fn of_with_length(item_count: usize, result_len: usize) -> Self
    where
        Self: Sized;

    /// next advances the iterator to a new combinatoric sequence and returns true.
    /// Returns false if there are no more sequences available.
    fn next(&mut self) -> bool;

    /// indices returns the combinatoric sequence of 0..item_count of result_len.
    fn indices(&self) -> &[usize];

    /// order_current uses .indices() to order elements from `src` in `dst`.
    fn order_current<T: Clone>(&self, src: &[T], dst: &mut [T]) {
        let indices = self.indices();
        assert!(dst.len() == indices.len());
        for (dst_idx, &src_idx) in indices.iter().enumerate() {
            dst[dst_idx] = src[src_idx].clone();
        }
    }
}

#[derive(Debug, Clone)]
pub struct Permutator {
    indices: Vec<usize>,
    cycles: Vec<usize>,
    started: bool,
}

impl CombinatoricIterator for Permutator {
    fn of_with_length(item_count: usize, permutations_length: usize) -> Self {
        assert!(permutations_length <= item_count);

        // Initialize indices
        let mut indices = Vec::with_capacity(item_count);
        for i in 0..item_count {
            indices.push(i);
        }

        // Initialize cycles
        let mut cycles = Vec::with_capacity(permutations_length);
        for i in 0..permutations_length {
            cycles.push(item_count - i);
        }

        Self {
            indices,
            cycles,
            started: false,
        }
    }

    #[inline]
    fn indices(&self) -> &[usize] {
        &self.indices[..self.cycles.len()]
    }

    fn next(&mut self) -> bool {
        if !self.started {
            self.started = true;
            return true;
        }

        let mut i = self.cycles.len() - 1;
        loop {
            self.cycles[i] -= 1;

            if self.cycles[i] == 0 {
                let indices_len = self.indices.len();
                let index_at_i = self.indices[i];

                self.indices.copy_within((i + 1)..indices_len, i);
                self.indices[indices_len - 1] = index_at_i;

                self.cycles[i] = indices_len - i;
            } else {
                let j = self.indices.len() - self.cycles[i];
                (self.indices[i], self.indices[j]) = (self.indices[j], self.indices[i]);
                return true;
            }

            if i == 0 {
                return false;
            }
            i -= 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Combinator {
    indices: Vec<usize>,
    item_count: usize,
    started: bool,
}

impl CombinatoricIterator for Combinator {
    fn of_with_length(item_count: usize, combinations_length: usize) -> Self {
        assert!(combinations_length <= item_count);

        // Initialize indices
        let mut indices = Vec::with_capacity(combinations_length);
        for i in 0..combinations_length {
            indices.push(i);
        }

        Self {
            indices,
            item_count,
            started: false,
        }
    }

    #[inline]
    fn indices(&self) -> &[usize] {
        &self.indices
    }

    fn next(&mut self) -> bool {
        if !self.started {
            self.started = true;
            return true;
        }

        let i = self
            .indices
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, &index)| {
                if index != i + self.item_count - self.indices.len() {
                    Some(i)
                } else {
                    None
                }
            });
        let i = match i {
            Some(i) => i,
            None => return false,
        };

        self.indices[i] += 1;
        for j in (i + 1)..self.indices.len() {
            self.indices[j] = self.indices[j - 1] + 1;
        }
        return true;
    }
}

#[derive(Debug, Clone)]
pub struct CombinatorWithReplacement {
    indices: Vec<usize>,
    item_count: usize,
    started: bool,
}

impl CombinatoricIterator for CombinatorWithReplacement {
    fn of_with_length(item_count: usize, combinations_length: usize) -> Self {
        assert!(combinations_length <= item_count);
        Self {
            indices: [0].repeat(combinations_length),
            item_count,
            started: false,
        }
    }

    #[inline]
    fn indices(&self) -> &[usize] {
        &self.indices
    }

    fn next(&mut self) -> bool {
        if !self.started {
            self.started = true;
            return true;
        }

        let i = self
            .indices
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, &index)| {
                if index != self.item_count - 1 {
                    Some(i)
                } else {
                    None
                }
            });
        let i = match i {
            Some(i) => i,
            None => return false,
        };

        let new = self.indices[i] + 1;
        self.indices[i..].fill(new);
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::combinatorics::CombinatorWithReplacement;

    use super::Combinator;
    use super::CombinatoricIterator;
    use super::Permutator;

    #[test]
    fn test_exact_length_permutations() {
        // Permutations of "012": "012", "021", "102", "120", "201", "210"
        let mut permutator = Permutator::of(3);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[0, 1, 2]);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[0, 2, 1]);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[1, 0, 2]);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[1, 2, 0]);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[2, 0, 1]);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[2, 1, 0]);

        assert!(!permutator.next());
    }

    #[test]
    fn test_shorter_permutations() {
        // 2-len permutations of "012": "01", "02", "10", "12", "20", "21"
        let mut permutator = Permutator::of_with_length(3, 2);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[0, 1]);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[0, 2]);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[1, 0]);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[1, 2]);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[2, 0]);

        assert!(permutator.next());
        assert_eq!(permutator.indices(), &[2, 1]);

        assert!(!permutator.next());
    }

    #[test]
    fn test_order_current() {
        let original = b"abc";
        let mut target: [u8; 3] = [0, 1, 2];

        let mut permutator = Permutator::of(original.len());

        assert!(permutator.next());
        permutator.order_current(original, &mut target);
        assert_eq!(&target, b"abc");

        assert!(permutator.next());
        permutator.order_current(original, &mut target);
        assert_eq!(&target, b"acb");

        assert!(permutator.next());
        permutator.order_current(original, &mut target);
        assert_eq!(&target, b"bac");

        assert!(permutator.next());
        permutator.order_current(original, &mut target);
        assert_eq!(&target, b"bca");

        assert!(permutator.next());
        permutator.order_current(original, &mut target);
        assert_eq!(&target, b"cab");

        assert!(permutator.next());
        permutator.order_current(original, &mut target);
        assert_eq!(&target, b"cba");

        assert!(!permutator.next());
    }

    #[test]
    fn test_exact_length_combinations() {
        // 3-item combinations of "012": "012"
        let mut combinator = Combinator::of(3);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 1, 2]);

        assert!(!combinator.next());
    }

    #[test]
    fn test_shorter_combinations() {
        // 2-length combinations of 0123: 01, 02, 03, 12, 13, 23
        let mut combinator = Combinator::of_with_length(4, 2);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 1]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 2]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 3]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[1, 2]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[1, 3]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[2, 3]);

        assert!(!combinator.next());
    }

    #[test]
    fn test_exact_length_combinations_with_replacement() {
        // 3-item combinations w/ repl. of 012: 000 001 002 011 012 022 111 112 122 222
        let mut combinator = CombinatorWithReplacement::of(3);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 0, 0]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 0, 1]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 0, 2]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 1, 1]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 1, 2]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 2, 2]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[1, 1, 1]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[1, 1, 2]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[1, 2, 2]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[2, 2, 2]);

        assert!(!combinator.next());
    }

    #[test]
    fn test_shorter_combinations_with_replacement() {
        // 2-length combinations of 0123: 00 01 02 03 11 12 13 22 23 33
        let mut combinator = CombinatorWithReplacement::of_with_length(4, 2);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 0]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 1]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 2]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[0, 3]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[1, 1]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[1, 2]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[1, 3]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[2, 2]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[2, 3]);

        assert!(combinator.next());
        assert_eq!(combinator.indices(), &[3, 3]);

        assert!(!combinator.next());
    }
}
