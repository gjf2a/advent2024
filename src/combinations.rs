pub struct ComboIterator<T: Clone, I: Iterator<Item = T> + Clone> {
    iter: I,
    entries: Vec<I>,
    prev: Option<Vec<T>>,
}

impl<T: Copy + Clone, I: Iterator<Item = T> + Clone> ComboIterator<T, I> {
    pub fn new(iter: I, num_entries: usize) -> Self {
        let mut entries = (0..num_entries).map(|_| iter.clone()).collect::<Vec<_>>();
        let start = entries
            .iter_mut()
            .map(|i| i.next().unwrap())
            .collect::<Vec<_>>();
        Self {
            iter,
            entries,
            prev: Some(start),
        }
    }

    fn advance(&mut self) {
        if let Some(prev) = &mut self.prev {
            for i in 0..self.entries.len() {
                match self.entries[i].next() {
                    Some(updated) => {
                        prev[i] = updated;
                        return;
                    }
                    None => {
                        self.entries[i] = self.iter.clone();
                        prev[i] = self.entries[i].next().unwrap();
                    }
                }
            }
            self.prev = None;
        }
    }
}

impl<T: Copy + Clone, I: Iterator<Item = T> + Clone> Iterator for ComboIterator<T, I> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.prev.clone();
        self.advance();
        result
    }
}

#[cfg(test)]
mod tests {
    use enum_iterator::all;

    use crate::multidim::ManhattanDir;

    use super::ComboIterator;

    #[test]
    fn combo_test() {
        let expected = vec![
            vec![ManhattanDir::N, ManhattanDir::N],
            vec![ManhattanDir::E, ManhattanDir::N],
            vec![ManhattanDir::S, ManhattanDir::N],
            vec![ManhattanDir::W, ManhattanDir::N],
            vec![ManhattanDir::N, ManhattanDir::E],
            vec![ManhattanDir::E, ManhattanDir::E],
            vec![ManhattanDir::S, ManhattanDir::E],
            vec![ManhattanDir::W, ManhattanDir::E],
            vec![ManhattanDir::N, ManhattanDir::S],
            vec![ManhattanDir::E, ManhattanDir::S],
            vec![ManhattanDir::S, ManhattanDir::S],
            vec![ManhattanDir::W, ManhattanDir::S],
            vec![ManhattanDir::N, ManhattanDir::W],
            vec![ManhattanDir::E, ManhattanDir::W],
            vec![ManhattanDir::S, ManhattanDir::W],
            vec![ManhattanDir::W, ManhattanDir::W],
        ];
        for (i, combo) in ComboIterator::new(all::<ManhattanDir>(), 2).enumerate() {
            assert_eq!(expected[i], combo);
        }
    }
}
