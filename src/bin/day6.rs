use std::collections::HashSet;

use advent2024::{
    chooser_main,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    Part,
};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let patrol_map = GridCharWorld::from_char_file(filename)?;
        println!(
            "{}",
            match part {
                Part::One => part1(&patrol_map),
                Part::Two => part2(&patrol_map),
            }
        );
        Ok(())
    })
}

fn part1(patrol_map: &GridCharWorld) -> usize {
    let mut guard = Guard::new(&patrol_map);
    while guard.go(&patrol_map) == InMap::Yes {}
    guard.path.len()
}

fn part2(patrol_map: &GridCharWorld) -> usize {
    let guard = Guard::new(&patrol_map);
    patrol_map
        .position_iter()
        .filter(|p| *p != guard.p)
        .filter(|p| {
            let mut candidate_map = patrol_map.clone();
            candidate_map.update(*p, '#');
            has_cycle(guard.clone(), &patrol_map)
        })
        .count()
}

fn has_cycle(mut guard: Guard, patrol_map: &GridCharWorld) -> bool {
    let guard_start = guard.clone();
    while guard.go(&patrol_map) == InMap::Yes {
        if guard.repeat(&guard_start) {
            return true;
        }
    }
    false
}

#[derive(Clone)]
struct Guard {
    p: Position,
    facing: ManhattanDir,
    path: HashSet<Position>,
}

impl Guard {
    fn new(world: &GridCharWorld) -> Self {
        let (p, _) = world
            .position_value_iter()
            .find(|(_, v)| **v == '^')
            .unwrap();
        let mut path = HashSet::new();
        path.insert(*p);
        Self {
            p: *p,
            facing: ManhattanDir::N,
            path,
        }
    }

    fn repeat(&self, prev_self: &Guard) -> bool {
        self.p == prev_self.p && self.facing == prev_self.facing
    }

    fn go(&mut self, world: &GridCharWorld) -> InMap {
        let ahead = self.facing.neighbor(self.p);
        match world.value(ahead) {
            None => InMap::No,
            Some(obstacle) => {
                match obstacle {
                    '#' => self.facing = self.facing.clockwise(),
                    _ => {
                        self.p = ahead;
                        self.path.insert(ahead);
                    }
                }
                InMap::Yes
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum InMap {
    Yes,
    No,
}
