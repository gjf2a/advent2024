use std::collections::HashMap;

use advent2024::{advent_main, all_lines, grid::GridCharWorld, multidim::{DirType, ManhattanDir, Position}, search_iter::BfsIter};
use common_macros::hash_map;

const NUMERIC_PAD: &str = "789
456
123
 0A";

const DIRECTION_PAD: &str = " ^A
<v>";

const NUM_ROBOTS: usize = 3;
const NUM_OUTPUTS: usize = 4;
const NUM_TABLES: usize = NUM_ROBOTS - 1;

/*
Base case: All As => 1
Recursive case: For every possible input < > ^ v A
* Create the previous state from which the input produces the current state
  * State is [direction, direction, digit]
  * previous state computed by:
    if < > ^ V, undo robot 1 arm
    if A, look at robot 2
       if < > ^ V, undo robot 2 arm
       if A, look at robot 3
          if < > ^ V, undo robot 3 arm (letter)
             If A, robot 3 just printed a character


* Recursively calculate the cost
* Find the minimum of the five costs.
 */

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        println!("{filename} {part:?}");
        let mut system = System::default();
        let part1 = all_lines(filename)?
            .map(|line| system.complexity(line.as_str()))
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
        Self { char2dir, dir2char, char2digit, digit2char }
    }
}

impl LookupTables {
    fn dir_for(&self, c: char) -> Position {
        *self.char2dir.get(&c).unwrap()
    }

    fn digit_for(&self, c: char) -> Position {
        *self.char2digit.get(&c).unwrap()
    }

    fn dir_key_for(&self, p: Position) -> char {
        *self.dir2char.get(&p).unwrap()
    }

    fn digit_key_for(&self, p: Position) -> char {
        *self.digit2char.get(&p).unwrap()
    }

    fn dir_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.char2dir.keys().copied()
    }

    fn digit_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.char2digit.keys().copied()
    }

    fn valid_digit(&self, p: Position) -> bool {
        self.digit2char.contains_key(&p)
    }

    fn valid_dir(&self, p: Position) -> bool {
        self.dir2char.contains_key(&p)
    }

    fn successors(&self, successor_map: &HashMap<Position,char>, current: Position) -> Vec<Position> {
        self.dir_chars().filter_map(|c| {
            match ManhattanDir::try_from(c) {
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
        }).collect()
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
    fn start(lookup: &LookupTables) -> Self {
        Self {
            arms: [lookup.dir_for('A'), lookup.dir_for('A'), lookup.digit_for('A')],
            outputs: [None; NUM_OUTPUTS],
        }
    }

    fn with_output(&self, c: char) -> Self {
        let mut i = 0;
        while self.outputs[i].is_none() {
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

    fn successors(&self, lookup: &LookupTables) -> Vec<Self> {
        let mut result = vec![];
        for arm0 in lookup.successor_dirs(self.arms[0]) {
            if arm0 == self.arms[0] {
                for arm1 in lookup.successor_dirs(self.arms[1]) {
                    if arm1 == self.arms[1] {
                        for arm2 in lookup.successor_digits(self.arms[2]) {
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
        result
    }
}

fn lookups(keypad_str: &str) -> (HashMap<char, Position>, HashMap<Position, char>) {
    let keypad = keypad_str.parse::<GridCharWorld>().unwrap();
    let lookup1 = keypad.position_value_iter().filter(|(p, v)| **v != ' ').map(|(p, v)| (*v, *p)).collect::<HashMap<_,_>>();
    let lookup2 = lookup1.iter().map(|(k, v)| (*v, *k)).collect();
    (lookup1, lookup2)
}

fn create_table() -> HashMap<Key, usize> {
    let lookup = LookupTables::default();
    let mut searcher = BfsIter::new(Key::start(&lookup), |state| {
        state.successors(&lookup)
    });

    searcher.all_depths()
}

struct System {
    direction: GridCharWorld, 
    digits: GridCharWorld,
    visited: HashMap<(Arms, Arms), usize>,
}

impl Default for System {
    fn default() -> Self {
        Self { 
            direction: DIRECTION_PAD.parse::<GridCharWorld>().unwrap(), 
            digits: NUMERIC_PAD.parse::<GridCharWorld>().unwrap(), 
            visited: Default::default() 
        }
    }
}

impl System {
    fn starting_arms(&self) -> Arms {
        self.goal_arms('A')
    }

    fn goal_arms(&self, c: char) -> Arms {
        let dir_a = self.direction.any_position_for('A');
        Arms {
            arms: [dir_a, dir_a, self.digits.any_position_for(c)]
        }
    }

    fn complexity(&mut self, target: &str) -> usize {
        let mut min_seq_length = 0;
        let mut arms = self.starting_arms();
        for goal in target.chars() {
            let goal = self.goal_arms(goal);
            min_seq_length += self.min_cost(arms, goal);
            arms = goal;
        }
        min_seq_length * target[..(target.len() - 1)].parse::<usize>().unwrap()
    }

    fn min_cost(&mut self, start: Arms, end: Arms) -> usize {
        if start == end {
            0
        } else {
            let mut min = None;
            for option in ['<', '>', '^', 'v', 'A'] {
                if let Some(prev_arms) = end.prev(option, &self.direction, &self.digits) {
                    let visited_key = (start, prev_arms);
                    println!("{visited_key:?}");
                    let value = match self.visited.get(&visited_key).copied() {
                        Some(value) => value,
                        None => {
                            let value = self.min_cost(start, prev_arms);
                            self.visited.insert(visited_key, value);
                            value
                        }
                    };
                    min = Some(min.map_or(value, |m| if value < m {value} else {m}));
                }
            }
            min.unwrap()
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Arms {
    arms: [Position; NUM_ROBOTS],
}

impl Arms {
    fn dir_back(c: char) -> Option<ManhattanDir> {
        ManhattanDir::try_from(c).ok().map(|d| d.inverse())
    }

    fn prev_position(c: char, p: Position, keypad: &GridCharWorld) -> Option<Position> {
        match Self::dir_back(c) {
            None => Some(p),
            Some(rev) => {
                let n = rev.neighbor(p);
                match keypad.value(n) {
                    None => None,
                    Some(v) => if v == ' ' {None} else {Some(n)}
                }
            }
        }
    }

    fn prev(&self, c: char, direction: &GridCharWorld, digits: &GridCharWorld) -> Option<Self> {
        match Self::prev_position(c, self.arms[0], direction) {
            None => None,
            Some(arm1) => {
                if arm1 == self.arms[0] {
                    match Self::prev_position(direction.value(self.arms[0]).unwrap(), self.arms[1], direction) {
                        None => None,
                        Some(arm2) => {
                            if arm2 == self.arms[1] {
                                match Self::prev_position(direction.value(self.arms[1]).unwrap(), self.arms[2], digits) {
                                    None => None,
                                    Some(arm3) => {
                                        Some(Self {
                                            arms: [arm1, arm2, arm3]
                                        })
                                    }
                                }
                            } else {
                                Some(Self {
                                    arms: [arm1, arm2, self.arms[2]]
                                })
                            }
                        }
                    }
                } else {
                    Some(Self {
                        arms: [arm1, self.arms[1], self.arms[2]]
                    })
                }
            }
        }
    }
}

fn min_cost(start: Arms, end: Arms, direction: &GridCharWorld, digits: &GridCharWorld) -> usize {
    if start == end {
        0
    } else {
        ['<', '>', '^', 'v', 'A']
            .iter()
            .filter_map(|c| end.prev(*c, direction, digits))
            .map(|prev_arms| min_cost(start, prev_arms, direction, digits))
            .min()
            .unwrap()
    }
}

struct Robots {
    keypads: [GridCharWorld; NUM_ROBOTS],
    presses_needed_for: [HashMap<(Position, Position), usize>; NUM_TABLES],
}

impl Robots {
    fn new() -> Self {
        let keypads = [
            DIRECTION_PAD.parse::<GridCharWorld>().unwrap(),
            DIRECTION_PAD.parse::<GridCharWorld>().unwrap(),
            NUMERIC_PAD.parse::<GridCharWorld>().unwrap(),
        ];
        let zero2one = hash_map!(
            ('A', 'A') => 1,
            ('A', '^') => 2,
            ('A', '>') => 2,
            ('A', 'v') => 3,
            ('A', '<') => 4,
            ('^', '^') => 1,
            ('^', '>') => 3,
            ('^', 'v') => 2,
            ('^', '<') => 3,
            ('>', '>') => 1,
            ('>', 'v') => 2,
            ('>', '<') => 3,
            ('<', '<') => 1,
            ('<', 'v') => 2,
            ('v', 'v') => 1,
        );
        let one2two = hash_map!(
            (('A', 'A'), 'A') => 1,
            (('^', 'A'), 'A') => 2,
            (('A', '^'), 'A') => 2,
            (('^', '^'), 'A') => 3,
        );
        let arms = keypads.clone().map(|p| p.any_position_for('A'));
        let starting_arms = Arms { arms };
        let mut tables = [HashMap::new(), HashMap::new()];
        for start in 0..tables.len() {}
        Self {
            keypads,
            presses_needed_for: tables,
        }
    }

    fn min_pushes_for(&self, level: usize, current: &Arms, goal: char) -> usize {
        let goal = self.keypads[level].any_position_for(goal);
        if level == 0 {
            1
        } else {
            let diff = current.arms[level] - goal;
            if diff.values().all(|v| v == 0) {
                self.min_pushes_for(level - 1, current, 'A')
            } else if diff[0] == 0 {
                if diff[1] > 0 {
                    self.min_pushes_for(level - 1, current, '^') + diff[1] as usize
                } else {
                    self.min_pushes_for(level - 1, current, 'v') + (-diff[1]) as usize
                }
            } else if diff[1] == 0 {
                if diff[0] > 0 {
                    self.min_pushes_for(level - 1, current, '<') + diff[0] as usize
                } else {
                    self.min_pushes_for(level - 1, current, '>') + (-diff[0]) as usize
                }
            } else {
                todo!()
            }
        }
    }
}

struct ArmMove {
    level: usize,
    button: char,
}

struct MoveToIterator {
    robots: Robots,
    level_goals: [char; NUM_ROBOTS],
    current_level: usize,
    state: Option<Arms>,
}

impl MoveToIterator {
    fn new(goal: char, arms: &Arms) -> Self {
        Self {
            robots: Robots::new(),
            level_goals: ['A', 'A', goal],
            current_level: 3,
            state: Some(arms.clone()),
        }
    }
    /*
    fn first_unaligned_level(&self) -> Option<usize> {
        (0..self.level_goals.len()).find(|level| self.robots.keypads[*level].value(self.state.arms[*level]).unwrap() != self.level_goals[*level])
    }
    */
}

impl Iterator for MoveToIterator {
    type Item = Arms;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.state.clone();

        result
    }
}
