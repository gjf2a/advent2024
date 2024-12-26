use std::collections::HashMap;

use advent2024::{
    advent_main, all_lines,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    search_iter::BfsIter,
    Part,
};

const NUMERIC_PAD: &str = "789
456
123
 0A";

const DIRECTION_PAD: &str = " ^A
<v>";

const NUM_OUTPUTS: usize = 4;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        println!("{filename} {part:?}");
        if options.contains(&"-2") {
            solve::<2>(filename)
        } else if options.contains(&"-4") {
            solve::<4>(filename)
        } else if options.contains(&"-5") {
            solve::<5>(filename)
        } else {
            match part {
                Part::One => solve::<3>(filename),
                Part::Two => solve::<26>(filename),
            }
        }
    })
}

fn solve<const NUM_ROBOTS: usize>(filename: &str) -> anyhow::Result<()> {
    let table = LookupTables::<NUM_ROBOTS>::default();
    let all_scores = table.find_all_scores();
    println!("# entries: {}", all_scores.len());
    let total = all_lines(filename)?
        .map(|line| {
            let min_cost = all_scores.get(&table.end_key(line.as_str()));
            let line_value = &line[0..(line.len() - 1)].parse::<usize>().unwrap();
            let min_cost = min_cost.copied().unwrap();
            println!(
                "{line}: {min_cost} * {line_value} = {}",
                min_cost * line_value
            );
            min_cost * line_value
        })
        .sum::<usize>();
    println!("{total}");
    Ok(())
}

struct LookupTables<const NUM_ROBOTS: usize> {
    char2dir: HashMap<char, Position>,
    dir2char: HashMap<Position, char>,
    char2digit: HashMap<char, Position>,
    digit2char: HashMap<Position, char>,
}

impl<const NUM_ROBOTS: usize> Default for LookupTables<NUM_ROBOTS> {
    fn default() -> Self {
        let (char2dir, dir2char) = lookups(DIRECTION_PAD);
        let (char2digit, digit2char) = lookups(NUMERIC_PAD);
        Self {
            char2dir,
            dir2char,
            char2digit,
            digit2char,
        }
    }
}

impl<const NUM_ROBOTS: usize> LookupTables<NUM_ROBOTS> {
    fn find_all_scores(&self) -> HashMap<Key<NUM_ROBOTS>, usize> {
        let mut searcher = BfsIter::new(self.start_key(), |state| state.successors(&self));
        searcher.by_ref().last();
        searcher.all_depths()
    }

    fn start_key(&self) -> Key<NUM_ROBOTS> {
        let mut arms = [self.dir_for('A'); NUM_ROBOTS];
        arms[arms.len() - 1] = self.digit_for('A');
        Key {
            arms: arms,
            outputs: [None; NUM_OUTPUTS],
        }
    }

    fn end_key(&self, goal: &str) -> Key<NUM_ROBOTS> {
        let mut result = self.start_key();
        for (i, c) in goal.char_indices() {
            result.outputs[i] = Some(c);
        }
        result
    }

    fn dir_for(&self, c: char) -> Position {
        *self.char2dir.get(&c).unwrap()
    }

    fn dir_key_for(&self, p: Position) -> char {
        *self.dir2char.get(&p).unwrap()
    }

    fn digit_for(&self, c: char) -> Position {
        *self.char2digit.get(&c).unwrap()
    }

    fn digit_key_for(&self, p: Position) -> char {
        *self.digit2char.get(&p).unwrap()
    }

    fn dir_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.char2dir.keys().copied()
    }

    fn push_level_ahead(
        &self,
        dir_key: char,
        current: Position,
        successor_map: &HashMap<Position, char>,
    ) -> Option<Position> {
        match ManhattanDir::try_from(dir_key) {
            Ok(d) => {
                let next = d.neighbor(current);
                if successor_map.contains_key(&next) {
                    Some(next)
                } else {
                    None
                }
            }
            Err(_) => Some(current),
        }
    }

    fn successor_dirs(&self, current: Position) -> Vec<Position> {
        self.dir_chars()
            .filter_map(|c| self.push_level_ahead(c, current, &self.dir2char))
            .collect()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Key<const NUM_ROBOTS: usize> {
    arms: [Position; NUM_ROBOTS],
    outputs: [Option<char>; NUM_OUTPUTS],
}

impl<const NUM_ROBOTS: usize> Key<NUM_ROBOTS> {
    fn with_output(&self, c: char) -> Self {
        let mut i = 0;
        while self.outputs[i].is_some() {
            i += 1;
        }
        let mut result = self.clone();
        result.outputs[i] = Some(c);
        result
    }

    fn replaced(&self, arm: usize, replacement: Position) -> Self {
        let mut result = self.clone();
        result.arms[arm] = replacement;
        result
    }

    fn is_complete(&self) -> bool {
        self.outputs.iter().all(|c| c.is_some())
    }

    fn successors(&self, lookup: &LookupTables<NUM_ROBOTS>) -> Vec<Self> {
        let mut result = vec![];
        if !self.is_complete() {
            for arm0 in lookup.successor_dirs(self.arms[0]) {
                if let Some(option) = self.deep_dive(lookup, arm0) {
                    result.push(option);
                }
            }
        }
        result
    }

    fn deep_dive(&self, lookup: &LookupTables<NUM_ROBOTS>, arm0: Position) -> Option<Self> {
        let mut n = 0;
        let mut current_arm = arm0;
        loop {
            if current_arm == self.arms[n] {
                if n < NUM_ROBOTS - 1 {
                    let decoder = if n < NUM_ROBOTS - 2 {&lookup.dir2char} else {&lookup.digit2char};
                    let next_arm = lookup.push_level_ahead(
                        lookup.dir_key_for(current_arm),
                        self.arms[n + 1],
                        &decoder,
                    );
                    match next_arm {
                        Some(next_arm) => {
                            n += 1;
                            current_arm = next_arm;
                        }
                        None => return None,
                    }
                } else {
                    return Some(self.with_output(lookup.digit_key_for(current_arm)));
                }
            } else {
                return Some(self.replaced(n, current_arm));
            }
        }
    }
}

fn lookups(keypad_str: &str) -> (HashMap<char, Position>, HashMap<Position, char>) {
    let keypad = keypad_str.parse::<GridCharWorld>().unwrap();
    let lookup1 = keypad
        .position_value_iter()
        .filter(|(_, v)| **v != ' ')
        .map(|(p, v)| (*v, *p))
        .collect::<HashMap<_, _>>();
    let lookup2 = lookup1.iter().map(|(k, v)| (*v, *k)).collect();
    (lookup1, lookup2)
}
