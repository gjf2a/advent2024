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
