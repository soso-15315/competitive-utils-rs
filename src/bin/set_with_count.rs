#![allow(dead_code)]

// hash set with a counter.
struct SetWithCount<T> {
    set: std::collections::HashSet<T>,
    counts: std::collections::HashMap<T, usize>,
}

impl<T: Eq + std::hash::Hash + Copy> SetWithCount<T> {
    fn new() -> Self {
        SetWithCount {
            set: std::collections::HashSet::new(),
            counts: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, x: T) {
        self.set.insert(x);
        *self.counts.entry(x).or_insert(0) += 1;
    }

    fn remove(&mut self, x: T) -> bool {
        if let Some(c) = self.counts.get_mut(&x) {
            *c -= 1;
            if *c == 0 {
                self.counts.remove(&x);
                self.set.remove(&x);
            }

            true
        } else {
            false
        }
    }

    fn contains(&self, x: T) -> bool {
        self.set.contains(&x)
    }

    fn get(&self, x: T) -> Option<&usize> {
        self.counts.get(&x)
    }

    fn len(&self) -> usize {
        self.set.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_with_count() {
        let mut set = SetWithCount::new();
        set.insert(1);
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert_eq!(set.len(), 3);
        assert_eq!(set.contains(1), true);
        assert_eq!(set.contains(2), true);
        assert_eq!(set.get(1), Some(&2));

        set.remove(1);
        assert_eq!(set.len(), 3);
        assert_eq!(set.contains(1), true);
        assert_eq!(set.get(1), Some(&1));

        set.remove(1);
        assert_eq!(set.len(), 2);
        assert_eq!(set.contains(1), false);
        assert_eq!(set.get(1), None);
    }
}

fn main() {}
