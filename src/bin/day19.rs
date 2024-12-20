use std::collections::BTreeSet;

use advent2024::{advent_main, all_lines};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, _part, _| {
        let mut input = all_lines(filename)?;
        let first_line = input.by_ref().next().unwrap();
        let towels = first_line.split(", ").collect::<Vec<_>>();
        let num_matches = input.skip(1).filter(|p| combo_for(p.as_str(), &towels).is_some()).count();
        println!("{num_matches}");
        Ok(())
    })
}

fn combo_for(pattern: &str, towels: &Vec<&str>) -> Option<BTreeSet<Combo>> {
    let combos = ComboIterator::new(pattern, towels);
    println!("{:?}", combos.prev_combos);
    Some(combos.prev_combos)
}

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Ord, Eq, Debug)]
struct Offset {
    char_position: usize,
    towel_len: usize,
    towel: usize,
}

impl Offset {
    fn after(&self) -> usize {
        self.char_position + self.towel_len
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug)]
struct Combo {
    towels: Vec<Offset>,
}

impl Combo {
    fn new(first_towel: Offset) -> Self {
        Self {towels: vec![first_towel]}
    }

    fn options(&self, towel_len: usize, pattern_len: usize) -> Vec<usize> {
        let mut result = vec![];
        let mut current_offset = 0;
        let mut i = 0;
        loop {
            while i < pattern_len && i == self.towels[current_offset].char_position {
                i = self.towels[current_offset].after();
                current_offset += 1;
            } 
            if i == pattern_len {
                break;
            }
            if current_offset == self.towels.len() {
                while i + towel_len < pattern_len {
                    result.push(i);
                }
                break;
            }
            while i < self.towels[current_offset].char_position {
                if i + towel_len <= self.towels[current_offset].char_position {
                    result.push(i);
                } 
            }
        }
        result
    }
}

struct ComboIterator {
    prev_combos: BTreeSet<Combo>,
    pattern: String,
    towels: Vec<String>,
}

impl ComboIterator {
    fn new(pattern: &str, towels: &Vec<&str>) -> Self {
        let mut prev_combos = BTreeSet::new();
        for (t, towel) in towels.iter().enumerate() {
            for i in 0..=(pattern.len() - towel.len()) {
                if &towel[..] == &pattern[i..i + towel.len()] {
                    let offset = Offset {char_position: i, towel_len: towel.len(), towel: t};
                    prev_combos.insert(Combo::new(offset));
                }
            }
        }
        let pattern = pattern.to_string();
        let towels = towels.iter().map(|s| s.to_string()).collect();
        Self { prev_combos, pattern, towels }
    }
}

impl Iterator for ComboIterator {
    type Item = BTreeSet<Combo>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev_combos.len() == 0 {
            None
        } else {

            todo!()
        }
    }
}