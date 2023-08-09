#[derive(Debug, Clone)]
pub struct Permutator {
    indices: Vec<usize>,
    cycles: Vec<usize>,
    started: bool,
}

impl Permutator {
    #[inline]
    pub fn of(item_count: usize) -> Self {
        Self::of_with_length(item_count, item_count)
    }

    pub fn of_with_length(item_count: usize, permutations_length: usize) -> Self {
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
    pub fn indices(&self) -> &[usize] {
        &self.indices[..self.cycles.len()]
    }

    pub fn order_current<T: Clone>(&self, from: &[T], target: &mut [T]) {
        assert!(target.len() == self.indices.len());
        for i in 0..self.indices.len() {
            target[i] = from[self.indices[i]].clone();
        }
    }

    pub fn next(&mut self) -> bool {
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

#[cfg(test)]
mod tests {
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
}
