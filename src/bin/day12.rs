use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use advent2024::{
    advent_main,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    searchers::{breadth_first_search, ContinueSearch, SearchQueue}, Part,
};
use enum_iterator::all;
use hash_histogram::HashHistogram;

// Part 1: New version works. Still wondering about why Labeler is incorrect.

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let garden = GridCharWorld::from_char_file(filename)?;
        let points2regions = bfs_points2regions(&garden);
        let regions = points2regions.values().copied().collect::<HashSet<_>>();
        let total = match part {
            Part::One => part1(&regions, &points2regions),
            Part::Two => part2(&points2regions),
        };
        println!("{total}");
        Ok(())
    })
}

fn part1(regions: &HashSet<usize>, points2regions: &HashMap<Position, usize>) -> usize {
    let mut areas = HashHistogram::<usize>::new();
    let mut perimeters = HashHistogram::new();
    for (p, label) in points2regions.iter() {
        areas.bump(label);
        perimeters.bump_by(label, edges(*p, &points2regions));
    }
    regions
        .iter()
        .map(|label| areas.count(&label) * perimeters.count(&label))
        .sum()
}

fn part2(points2regions: &HashMap<Position, usize>) -> usize {
    
    todo!();
}

fn edges(p: Position, points2regions: &HashMap<Position, usize>) -> usize {
    let label = points2regions.get(&p).unwrap();
    all::<ManhattanDir>()
        .filter(|d| {
            points2regions
                .get(&d.neighbor(p))
                .map_or(true, |r| r != label)
        })
        .count()
}

fn bfs_points2regions(garden: &GridCharWorld) -> HashMap<Position, usize> {
    let mut current = 0;
    let mut result = HashMap::new();
    for (p, v) in garden.position_value_iter() {
        if !result.contains_key(p) {
            breadth_first_search(p, |s, q| {
                result.insert(*s, current);
                for d in all::<ManhattanDir>() {
                    let n = d.neighbor(*s);
                    if garden.value(n).map_or(false, |c| c == *v) {
                        q.enqueue(&n);
                    }
                }
                ContinueSearch::Yes
            });
            current += 1;
        }
    }
    result
}

fn points2regions(garden: &GridCharWorld) -> HashMap<Position, usize> {
    let mut result = HashMap::new();
    let mut equivalencies = Labeler::default();
    for (p, v) in garden.position_value_iter() {
        let n_char_match = char_match_label(*v, ManhattanDir::N.neighbor(*p), garden, &result);
        let w_char_match = char_match_label(*v, ManhattanDir::W.neighbor(*p), garden, &result);
        let label = match w_char_match {
            None => match n_char_match {
                None => equivalencies.new_label(),
                Some(l) => l,
            },
            Some(wl) => {
                if let Some(nl) = n_char_match {
                    equivalencies.mark_equal(nl, wl);
                }
                wl
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
        let keep = min(self.get(label1), self.get(label2));
        self.equivalencies[label1] = keep;
        self.equivalencies[label2] = keep;
    }

    fn get(&self, label: usize) -> usize {
        if self.equivalencies[label] == label {
            label
        } else {
            self.get(self.equivalencies[label])
        }
    }
}
