use std::collections::{BTreeMap, BTreeSet};

use advent2024::{advent_main, all_lines};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, _part, _| {
        let mut input = all_lines(filename)?;
        let first_line = input.by_ref().next().unwrap();
        let towels = first_line.split(", ").collect::<Vec<_>>();
        let num_matches = input
            .skip(1)
            .inspect(|line| println!("Checking {line}..."))
            .filter(|p| Table::new(p.as_str(), &towels).part1())
            .inspect(|_| println!("match!"))
            .count();
        println!("{num_matches}");
        Ok(())
    })
}

struct Table {
    pos2towels: Vec<Vec<usize>>,
    towel_lengths: Vec<usize>,
}

impl Table {
    fn new(pattern: &str, towels: &Vec<&str>) -> Self {
        let towel_lengths = towels.iter().map(|t| t.len()).collect();
        let pos2towels = (0..pattern.len())
            .map(|i| {
                towels
                    .iter()
                    .enumerate()
                    .filter(|(_, towel)| {
                        i + towel.len() <= pattern.len()
                            && &towel[..] == &pattern[i..i + towel.len()]
                    })
                    .map(|(t, _)| t)
                    .collect()
            })
            .collect();

        Self {
            pos2towels,
            towel_lengths,
        }
    }

    fn part1(&self) -> bool {
        let mut solutions = BTreeSet::new();
        solutions.insert(self.pos2towels.len());
        for p in (0..self.pos2towels.len()).rev() {
            for towel in self.pos2towels[p].iter() {
                let successor = p + self.towel_lengths[*towel];
                if solutions.contains(&successor) {
                    solutions.insert(p);
                }
            }
        }
        solutions.contains(&0)
    }
}

fn combo_for(pattern: &str, towels: &Vec<&str>) -> Option<Combo> {
    for cs in ComboIterator::new(pattern, towels) {
        for c in cs {
            if c.complete() {
                return Some(c);
            }
        }
    }
    None
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug)]
struct Combo {
    available_indices: BTreeSet<usize>,
    towels_used: BTreeSet<usize>,
}

impl Combo {
    fn new(pattern_len: usize) -> Self {
        Self {
            available_indices: (0..pattern_len).collect(),
            towels_used: BTreeSet::new(),
        }
    }

    fn can_add_towel(&self, towel_start: usize, towel_len: usize) -> bool {
        (towel_start..towel_start + towel_len).all(|i| self.available_indices.contains(&i))
    }

    fn add_towel(&mut self, towel_id: usize, towel_start: usize, towel_len: usize) {
        assert!(self.can_add_towel(towel_start, towel_len));
        self.towels_used.insert(towel_id);
        for i in towel_start..towel_start + towel_len {
            self.available_indices.remove(&i);
        }
    }

    fn complete(&self) -> bool {
        self.available_indices.is_empty()
    }
}

struct ComboIterator {
    prev_combos: BTreeSet<Combo>,
    towel2matches: Vec<Vec<usize>>,
    towel_lengths: Vec<usize>,
}

impl ComboIterator {
    fn new(pattern: &str, towels: &Vec<&str>) -> Self {
        let mut prev_combos = BTreeSet::new();
        let mut towel2matches = vec![];
        let mut towel_lengths = vec![];
        for (t, towel) in towels.iter().enumerate() {
            towel_lengths.push(towel.len());
            let mut towel_matches = vec![];
            for i in 0..=(pattern.len() - towel.len()) {
                if &towel[..] == &pattern[i..i + towel.len()] {
                    towel_matches.push(i);
                    let mut combo = Combo::new(pattern.len());
                    combo.add_towel(t, i, towel.len());
                    prev_combos.insert(combo);
                }
            }
            towel2matches.push(towel_matches);
        }
        println!("Ready!");
        Self {
            prev_combos,
            towel2matches,
            towel_lengths,
        }
    }
}

impl Iterator for ComboIterator {
    type Item = BTreeSet<Combo>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev_combos.len() == 0 {
            None
        } else {
            let mut current_combos = BTreeSet::new();
            for combo in self.prev_combos.iter() {
                for (towel_id, matches) in self.towel2matches.iter().enumerate() {
                    for towel_start in matches.iter() {
                        if combo.can_add_towel(*towel_start, self.towel_lengths[towel_id]) {
                            let mut new_combo = combo.clone();
                            new_combo.add_towel(
                                towel_id,
                                *towel_start,
                                self.towel_lengths[towel_id],
                            );
                            current_combos.insert(new_combo);
                        }
                    }
                }
            }
            std::mem::swap(&mut current_combos, &mut self.prev_combos);
            println!("returning {} combos", current_combos.len());
            Some(current_combos)
        }
    }
}
