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
        let winner = table.by_ref().skip_while(|r| r.p != exit).next().unwrap();
        println!("{}", winner.score);
        Ok(())
    })
}

struct ReindeerPathTable {
    candidates: PriorityQueue<Reindeer, isize>,
    maze: GridCharWorld,
    visited: HashSet<Reindeer>,
    exit: Position
}

impl ReindeerPathTable {
    fn new(maze: GridCharWorld) -> Self {
        let mut candidates = PriorityQueue::new();
        candidates.push(
            Reindeer {
                score: 0,
                p: maze.any_position_for('S'),
                f: ManhattanDir::E,
            },
            0,
        );
        let visited = HashSet::new();
        let exit = maze.any_position_for('E');
        Self { maze, candidates, visited, exit }
    }

    fn dequeue(&mut self) -> Option<Reindeer> {
        let result;
        loop {
            match self.candidates.pop() {
                None => return None,
                Some((r, _)) => {
                    if !self.visited.contains(&r) {
                        self.visited.insert(r);
                        result = Some(r);
                        break;
                    }
                }
            }
        }

        result
    }
}

impl Iterator for ReindeerPathTable {
    type Item = Reindeer;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.dequeue();
        if let Some(r) = result {
            for candidate in r.futures() {
                if self.maze.value(candidate.p).unwrap() != '#' {
                    match self.candidates.get_priority(&candidate) {
                        None => {
                            self.candidates.push(candidate, -candidate.score);
                        }
                        Some(priority) => {
                            let new_priority = -candidate.score;
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
    score: isize,
    p: Position,
    f: ManhattanDir,
}

impl Reindeer {
    fn futures(&self) -> Vec<Reindeer> {
        vec![
            Self {score: self.score + 1, p: self.f.neighbor(self.p), f: self.f},
            Self {score: self.score + 1000, p: self.p, f: self.f.clockwise()},
            Self {score: self.score + 1000, p: self.p, f: self.f.counterclockwise()}
        ]
    }
}
