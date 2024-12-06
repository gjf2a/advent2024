use std::collections::HashSet;

use advent2024::{chooser_main, grid::GridCharWorld, multidim::{DirType, ManhattanDir, Position}};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let patrol_map = GridCharWorld::from_char_file(filename)?;
        let mut guard = Guard::new(&patrol_map);
        while guard.go(&patrol_map) == InMap::Yes {}
        println!("{}", guard.path.len());
        Ok(())
    })
}

#[derive(Clone)]
struct Guard {
    p: Position,
    facing: ManhattanDir,
    path: HashSet<Position>
}

impl Guard {
    fn new(world: &GridCharWorld) -> Self {
        let (p, _) = world.position_value_iter().find(|(_, v)| **v == '^').unwrap();
        let mut path = HashSet::new();
        path.insert(*p);
        Self {p: *p, facing: ManhattanDir::N, path}
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
    Yes, No
}