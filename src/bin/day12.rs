use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use advent2024::{
    advent_main,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
};
use enum_iterator::all;
use hash_histogram::HashHistogram;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, _part, _| {
        let garden = GridCharWorld::from_char_file(filename)?;
        let points2regions = points2regions(&garden);
        let regions = points2regions.values().copied().collect::<HashSet<_>>();
        let mut areas = HashHistogram::<_, usize>::new();
        let mut perimeters = HashHistogram::new();
        for (p, label) in points2regions.iter() {
            areas.bump(label);
            perimeters.bump_by(
                label,
                all::<ManhattanDir>()
                    .filter(|d| {
                        points2regions
                            .get(&d.neighbor(*p))
                            .map_or(true, |r| r != label)
                    })
                    .count(),
            );
        }
        for label in regions.iter() {
            println!("{label}:\t{}, {}, {}", areas.count(label), perimeters.count(label), areas.count(label) * perimeters.count(label));
        }
        let total = regions
            .iter()
            .map(|label| areas.count(&label) * perimeters.count(&label))
            .sum::<usize>();
        println!("{total}");
        Ok(())
    })
}

fn points2regions(garden: &GridCharWorld) -> HashMap<Position, usize> {
    let mut result = HashMap::new();
    let mut equivalencies = Labeler::default();
    for (p, v) in garden.position_value_iter() {
        let n_char_match = char_match_label(*v, ManhattanDir::N.neighbor(*p), garden, &result);
        let w_char_match = char_match_label(*v, ManhattanDir::W.neighbor(*p), garden, &result);
        let label = match n_char_match {
            None => match w_char_match {
                None => equivalencies.new_label(),
                Some(l) => l,
            },
            Some(nl) => {
                if let Some(wl) = w_char_match {
                    equivalencies.mark_equal(nl, wl);
                }
                nl
            }
        };
        result.insert(*p, label);
    }
    result
        .iter()
        .map(|(k, v)| (*k, equivalencies.get(*v)))
        .collect()
}

fn char_match_label(
    c: char,
    n: Position,
    garden: &GridCharWorld,
    labels: &HashMap<Position, usize>,
) -> Option<usize> {
    garden
        .value(n)
        .filter(|nc| *nc == c)
        .map(|_| labels.get(&n).copied().unwrap())
}

#[derive(Clone, Default)]
struct Labeler {
    equivalencies: Vec<usize>,
}

impl Labeler {
    fn new_label(&mut self) -> usize {
        let result = self.equivalencies.len();
        self.equivalencies.push(result);
        result
    }

    fn mark_equal(&mut self, label1: usize, label2: usize) {
        let keep = min(label1, label2);
        let redirect = max(label1, label2);
        self.equivalencies[redirect] = keep;
    }

    fn get(&self, label: usize) -> usize {
        self.equivalencies[label]
    }
}
