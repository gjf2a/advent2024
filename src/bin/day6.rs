use std::collections::HashSet;

use advent2024::{
    chooser_main,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    Part,
};
use common_macros::hash_set;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let patrol_map = GridCharWorld::from_char_file(filename)?;
        println!(
            "{}",
            match part {
                Part::One => part1(&patrol_map),
                Part::Two => todo!()//part2(&patrol_map),
            }
        );
        Ok(())
    })
}

fn part1(patrol_map: &GridCharWorld) -> usize {
    let guard = Guard::new(patrol_map);
    let mut visited = hash_set!(guard.p);
    for g in (GuardIterator {guard, patrol_map}) {
        visited.insert(g.p);
    }
    visited.len()
}
/*
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
*/
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Guard {
    p: Position,
    facing: ManhattanDir,
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
        }
    }

    fn repeat(&self, prev_self: &Guard) -> bool {
        self.p == prev_self.p && self.facing == prev_self.facing
    }
}

struct GuardIterator<'a> {
    guard: Guard,
    patrol_map: &'a GridCharWorld,
}

impl<'a> Iterator for GuardIterator<'a> {
    type Item = Guard;
    
    fn next(&mut self) -> Option<Self::Item> {
        let ahead = self.guard.facing.neighbor(self.guard.p);
        self.patrol_map.value(ahead).map(|ahead_value| {
            match ahead_value {
                '#' => self.guard.facing = self.guard.facing.clockwise(),
                _ => self.guard.p = ahead,
            }
            self.guard
        })
    }
}