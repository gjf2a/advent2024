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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug)]
struct Combo {
    towels: Vec<Offset>,
}

impl Combo {
    fn new(first_towel: Offset) -> Self {
        Self {towels: vec![first_towel]}
    }
}

struct ComboIterator {
    prev_combos: BTreeSet<Combo>
}

impl ComboIterator {
    fn new(pattern: &str, towels: &Vec<&str>) -> Self {
        let mut prev_combos = BTreeSet::new();
        for (t, towel) in towels.iter().enumerate() {
            for i in 0..=(pattern.len() - towel.len()) {
                //println!("{:?} vs {:?}", &towel[..], &pattern[i..i + towel.len()]);
                if &towel[..] == &pattern[i..i + towel.len()] {
                    //println!("match");
                    let offset = Offset {char_position: i, towel_len: towel.len(), towel: t};
                    prev_combos.insert(Combo::new(offset));
                }
            }
        }
        Self { prev_combos }
    }
}

impl Iterator for ComboIterator {
    type Item = BTreeSet<Combo>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}