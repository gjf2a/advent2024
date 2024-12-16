use std::collections::HashSet;

use advent2024::{
    advent_main,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
};
use priority_queue::PriorityQueue;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, _part, _| {
        let mut table = ReindeerPathTable::new(GridCharWorld::from_char_file(filename)?);
        let exit = table.exit;
        let (winner, score) = table
            .by_ref()
            .skip_while(|(r, _)| r.p != exit)
            .next()
            .unwrap();
        assert_eq!(winner.p, exit);
        println!("{}", score);
        Ok(())
    })
}

struct ReindeerPathTable {
    candidates: PriorityQueue<Reindeer, isize>,
    maze: GridCharWorld,
    visited: HashSet<Reindeer>,
    exit: Position,
}

impl ReindeerPathTable {
    fn new(maze: GridCharWorld) -> Self {
        let mut candidates = PriorityQueue::new();
        candidates.push(
            Reindeer {
                p: maze.any_position_for('S'),
                f: ManhattanDir::E,
            },
            0,
        );
        let visited = HashSet::new();
        let exit = maze.any_position_for('E');
        Self {
            maze,
            candidates,
            visited,
            exit,
        }
    }

    fn dequeue(&mut self) -> Option<(Reindeer, isize)> {
        let result;
        loop {
            match self.candidates.pop() {
                None => return None,
                Some((r, score)) => {
                    if !self.visited.contains(&r) {
                        self.visited.insert(r);
                        result = Some((r, -score));
                        break;
                    }
                }
            }
        }

        result
    }
}

impl Iterator for ReindeerPathTable {
    type Item = (Reindeer, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.dequeue();
        if let Some((r, s)) = result {
            for (candidate, score) in r.futures(s) {
                if self.maze.value(candidate.p).unwrap() != '#' {
                    let new_priority = -score;
                    match self.candidates.get_priority(&candidate) {
                        None => {
                            self.candidates.push(candidate, new_priority);
                        }
                        Some(priority) => {
                            if new_priority > *priority {
                                self.candidates.change_priority(&candidate, new_priority);
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Reindeer {
    p: Position,
    f: ManhattanDir,
}

impl Reindeer {
    fn futures(&self, score: isize) -> Vec<(Reindeer, isize)> {
        vec![
            (
                Self {
                    p: self.f.neighbor(self.p),
                    f: self.f,
                },
                score + 1,
            ),
            (
                Self {
                    p: self.p,
                    f: self.f.clockwise(),
                },
                score + 1000,
            ),
            (
                Self {
                    p: self.p,
                    f: self.f.counterclockwise(),
                },
                score + 1000,
            ),
        ]
    }
}
