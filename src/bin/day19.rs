use std::collections::BTreeSet;

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

