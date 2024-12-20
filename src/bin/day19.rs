use advent2024::{advent_main, all_lines, Part};
use hash_histogram::HashHistogram;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let mut input = all_lines(filename)?;
        let first_line = input.by_ref().next().unwrap();
        let towels = first_line.split(", ").collect::<Vec<_>>();
        match part {
            Part::One => {
                let num_matches = input
                    .skip(1)
                    .filter(|p| Table::new(p.as_str(), &towels).solve() > 0)
                    .count();
                println!("{num_matches}");
            }
            Part::Two => {
                let total_matches = input
                    .skip(1)
                    .map(|p| Table::new(p.as_str(), &towels).solve())
                    .sum::<usize>();
                println!("{total_matches}");
            }
        }
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

    fn solve(&self) -> usize {
        let mut counts: HashHistogram<usize> = HashHistogram::new();
        for p in (0..self.pos2towels.len()).rev() {
            for towel in self.pos2towels[p].iter() {
                let successor = p + self.towel_lengths[*towel];
                if successor == self.pos2towels.len() {
                    counts.bump(&p);
                } else {
                    counts.bump_by(&p, counts.count(&successor));
                }
            }
        }
        counts.count(&0)
    }
}
