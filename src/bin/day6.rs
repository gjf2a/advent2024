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
        let result = match part {
            Part::One => part1(&patrol_map),
            Part::Two => part2(&patrol_map),
        };
        println!("{result}");
        Ok(())
    })
}

fn part1(patrol_map: &GridCharWorld) -> usize {
    Guard::new(patrol_map)
        .travel_to_exit(patrol_map)
        .map(|g| g.at)
        .collect::<HashSet<_>>()
        .len()
}

fn part2(patrol_map: &GridCharWorld) -> usize {
    let mut alternate_world = patrol_map.clone();
    let mut cyclic_barriers = HashSet::new();
    let guard = Guard::new(patrol_map);
    for pose in guard.travel_to_exit(patrol_map).skip(1) {
        if !cyclic_barriers.contains(&pose.at) {
            alternate_world.update(pose.at, '#');
            if has_cycle(&alternate_world, guard) {
                cyclic_barriers.insert(pose.at);
            }
            alternate_world.update(pose.at, '.');
        }
    }
    cyclic_barriers.len()
}

fn has_cycle(patrol_map: &GridCharWorld, guard: Guard) -> bool {
    let mut visited = HashSet::new();
    for g in guard.travel_to_exit(patrol_map) {
        if visited.contains(&g) {
            return true;
        }
        visited.insert(g);
    }
    false
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Guard {
    at: Position,
    facing: ManhattanDir,
}

impl Guard {
    fn new(world: &GridCharWorld) -> Self {
        let (p, _) = world
            .position_value_iter()
            .find(|(_, v)| **v == '^')
            .unwrap();
        Self {
            at: *p,
            facing: ManhattanDir::N,
        }
    }

    fn travel_to_exit<'a>(&self, patrol_map: &'a GridCharWorld) -> GuardTravelIterator<'a> {
        GuardTravelIterator {
            guard: Some(*self),
            patrol_map,
        }
    }
}

struct GuardTravelIterator<'a> {
    guard: Option<Guard>,
    patrol_map: &'a GridCharWorld,
}

impl<'a> Iterator for GuardTravelIterator<'a> {
    type Item = Guard;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.guard;
        self.guard = self
            .guard
            .map(|g| (g, g.facing.neighbor(g.at)))
            .and_then(|(g, ahead)| {
                self.patrol_map
                    .value(ahead)
                    .map(|ahead_value| match ahead_value {
                        '#' => Guard {
                            at: g.at,
                            facing: g.facing.clockwise(),
                        },
                        _ => Guard {
                            at: ahead,
                            facing: g.facing,
                        },
                    })
            });
        prev
    }
}
