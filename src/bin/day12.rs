use std::{cmp::{max, min}, collections::{HashMap, HashSet}};

use advent2024::{advent_main, grid::GridCharWorld, multidim::{DirType, ManhattanDir, Position}};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let garden = GridCharWorld::from_char_file(filename)?;
        let points2regions = points2regions(&garden);
        Ok(())
    })
}

fn points2regions(garden: &GridCharWorld) -> HashMap<Position, usize> {
    let mut result = HashMap::new();
    let mut equivalencies = Labeler::default();
    for (p, v) in garden.position_value_iter() {
        let n = ManhattanDir::N.neighbor(*p);
        let w = ManhattanDir::W.neighbor(*p);
        let n_char_match = garden.value(n).filter(|nc| nc == v).map(|_| result.get(&n).copied().unwrap());
        let w_char_match = garden.value(w).filter(|wc| wc == v).map(|_| result.get(&w).copied().unwrap());
        let label = match n_char_match {
            None => match w_char_match {
                None => equivalencies.new_label(),
                Some(l) => l,
            }
            Some(nl) => {
                if let Some(wl) = w_char_match {
                    equivalencies.mark_equal(nl, wl);
                }
                nl
            }
        };
        result.insert(*p, label);
    }
    todo!("Resolve equivalencies");
    result
}

#[derive(Clone, Default)]
struct Labeler {
    equivalencies: Vec<usize>
}

impl Labeler {
    fn new_label(&mut self) -> usize {
        let result = self.equivalencies.len();
        self.equivalencies.push(result);
        result
    }

    fn mark_equal(&mut self, label1: usize, label2: usize) {
        assert!(label1 < label2);
        self.equivalencies[label2] = label1;
    }
}