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
                Part::Two => todo!(), //part2(&patrol_map),
            }
        );
        Ok(())
    })
}

fn part1(patrol_map: &GridCharWorld) -> usize {
    GuardIterator {
        guard: Some(Guard::new(patrol_map)),
        patrol_map,
    }
    .map(|g| g.p)
    .collect::<HashSet<_>>()
    .len()
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
}

struct GuardIterator<'a> {
    guard: Option<Guard>,
    patrol_map: &'a GridCharWorld,
}

impl<'a> Iterator for GuardIterator<'a> {
    type Item = Guard;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.guard;
        self.guard = self
            .guard
            .map(|g| (g, g.facing.neighbor(g.p)))
            .and_then(|(g, ahead)| {
                self.patrol_map
                    .value(ahead)
                    .map(|ahead_value| match ahead_value {
                        '#' => Guard {
                            p: g.p,
                            facing: g.facing.clockwise(),
                        },
                        _ => Guard {
                            p: ahead,
                            facing: g.facing,
                        },
                    })
            });
        prev
    }
}
