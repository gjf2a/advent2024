use advent2024::{advent_main, grid::GridCharWorld, multidim::Position};

const NUMERIC_PAD: &str = "789
456
123
 0A";

const DIRECTION_PAD: &str = " ^A
<v>";

const LEVELS: usize = 4;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        println!("{filename} {part:?}");
        let robots = Robots::new();

        Ok(())
    })
}

struct Robots {
    keypads: [GridCharWorld; LEVELS],
    
}

impl Robots {
    fn new() -> Self {
        Self {
            keypads: [
                DIRECTION_PAD.parse::<GridCharWorld>().unwrap(), 
                DIRECTION_PAD.parse::<GridCharWorld>().unwrap(), 
                DIRECTION_PAD.parse::<GridCharWorld>().unwrap(), 
                NUMERIC_PAD.parse::<GridCharWorld>().unwrap(),
                ]
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

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Arms {
    arms: [Position; LEVELS],
}

struct MoveToIterator {
    robots: Robots,
    level_goals: [char; LEVELS],
    current_level: usize,
    state: Option<Arms>,
}

impl MoveToIterator {
    fn new(goal: char, arms: &Arms) -> Self {
        Self {
            robots: Robots::new(),
            level_goals: ['A', 'A', 'A', goal],
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