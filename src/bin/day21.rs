use std::collections::HashMap;

use advent2024::{advent_main, grid::GridCharWorld, multidim::Position, search_iter::BfsIter};
use common_macros::hash_map;

const NUMERIC_PAD: &str = "789
456
123
 0A";

const DIRECTION_PAD: &str = " ^A
<v>";

const NUM_ROBOTS: usize = 3;
const NUM_TABLES: usize = NUM_ROBOTS - 1;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        println!("{filename} {part:?}");
        let robots = Robots::new();

        Ok(())
    })
}

struct Robots {
    keypads: [GridCharWorld; NUM_ROBOTS],
    presses_needed_for: [HashMap<(Position,Position), usize>; NUM_TABLES]
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
        let starting_arms = Arms {arms};
        let mut tables = [HashMap::new(), HashMap::new()];
        BfsIter::new(start, successor)
        for start in 0..tables.len() {

        }
        Self {
            keypads,
            presses_needed_for: tables
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

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Arms {
    arms: [Position; NUM_ROBOTS],
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

    fn first_unaligned_level(&self) -> Option<usize> {
        (0..self.level_goals.len()).find(|level| self.robots.keypads[*level].value(self.state.arms[*level]).unwrap() != self.level_goals[*level])
    }
}

impl Iterator for MoveToIterator {
    type Item = Arms;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.state.clone();

        result
    }
}