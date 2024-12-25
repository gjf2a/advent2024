use std::collections::HashMap;

use advent2024::{
    advent_main, all_lines,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    search_iter::BfsIter,
};

const NUMERIC_PAD: &str = "789
456
123
 0A";

const DIRECTION_PAD: &str = " ^A
<v>";

const NUM_ROBOTS: usize = 3;
const NUM_OUTPUTS: usize = 4;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        println!("{filename} {part:?}");
        let table = LookupTables::default();
        let all_scores = table.find_all_scores();
        println!("# entries: {}", all_scores.len());
        let part1 = all_lines(filename)?
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
        println!("{part1}");
        Ok(())
    })
}

struct LookupTables {
    char2dir: HashMap<char, Position>,
    dir2char: HashMap<Position, char>,
    char2digit: HashMap<char, Position>,
    digit2char: HashMap<Position, char>,
}

impl Default for LookupTables {
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

impl LookupTables {
    fn find_all_scores(&self) -> HashMap<Key, usize> {
        let mut searcher = BfsIter::new(self.start_key(), |state| state.successors(&self));
        searcher.by_ref().last();
        searcher.all_depths()
    }

    fn start_key(&self) -> Key {
        Key {
            arms: [self.dir_for('A'), self.dir_for('A'), self.digit_for('A')],
            outputs: [None; NUM_OUTPUTS],
        }
    }

    fn end_key(&self, goal: &str) -> Key {
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

    fn successors(
        &self,
        successor_map: &HashMap<Position, char>,
        current: Position,
    ) -> Vec<Position> {
        self.dir_chars()
            .filter_map(|c| self.push_level_ahead(c, current, successor_map))
            .collect()
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
        self.successors(&self.dir2char, current)
    }

    fn successor_digits(&self, current: Position) -> Vec<Position> {
        self.successors(&self.digit2char, current)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Key {
    arms: [Position; NUM_ROBOTS],
    outputs: [Option<char>; NUM_OUTPUTS],
}

impl Key {
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

    fn successors(&self, lookup: &LookupTables) -> Vec<Self> {
        let mut result = vec![];
        if !self.is_complete() {
            for arm0 in lookup.successor_dirs(self.arms[0]) {
                if arm0 == self.arms[0] {
                    let arm1 = lookup.push_level_ahead(
                        lookup.dir_key_for(arm0),
                        self.arms[1],
                        &lookup.dir2char,
                    );
                    if let Some(arm1) = arm1 {
                        if arm1 == self.arms[1] {
                            let arm2 = lookup.push_level_ahead(
                                lookup.dir_key_for(arm1),
                                self.arms[2],
                                &lookup.digit2char,
                            );
                            if let Some(arm2) = arm2 {
                                if arm2 == self.arms[2] {
                                    result.push(self.with_output(lookup.digit_key_for(arm2)));
                                } else {
                                    result.push(self.replaced(2, arm2));
                                }
                            }
                        } else {
                            result.push(self.replaced(1, arm1));
                        }
                    }
                } else {
                    result.push(self.replaced(0, arm0));
                }
            }
        }
        result
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
