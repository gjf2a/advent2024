use std::collections::{HashMap, HashSet};

use advent2024::{
    advent_main, grid::GridCharWorld, multidim::{DirType, ManhattanDir, Position}, searchers::{breadth_first_search, ContinueSearch, SearchQueue}, Part
};
use multimap::MultiMap;
use priority_queue::PriorityQueue;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let mut table = ReindeerPathTable::new(GridCharWorld::from_char_file(filename)?);
        let exit = table.exit;
        let (winner, score) = table
            .by_ref()
            .skip_while(|(r, _)| r.p != exit)
            .next()
            .unwrap();
        assert_eq!(winner.p, exit);
        match part {
            Part::One => {
                println!("{}", score);
            }
            Part::Two => {
                while let Some(_) = table.next() {}
                assert_eq!(table.num_empty_squares(), table.parents.keys().count());
                let on_path = table.visited_towards_exit();
                println!("{}", on_path.len());
            }
        }

        Ok(())
    })
}

struct ReindeerPathTable {
    candidates: PriorityQueue<Reindeer, isize>,
    maze: GridCharWorld,
    visited: HashMap<Reindeer, isize>,
    parents: MultiMap<Position, Position>,
    entrance: Position,
    exit: Position,
}

impl ReindeerPathTable {
    fn new(maze: GridCharWorld) -> Self {
        let mut candidates = PriorityQueue::new();
        let entrance = maze.any_position_for('S');
        candidates.push(
            Reindeer {
                p: entrance,
                f: ManhattanDir::E,
            },
            0,
        );
        let exit = maze.any_position_for('E');
        Self {
            maze,
            candidates,
            visited: HashMap::new(),
            parents: MultiMap::new(),
            entrance,
            exit,
        }
    }

    fn num_empty_squares(&self) -> usize {
        self.maze
            .position_value_iter()
            .filter(|(_, v)| **v != '#')
            .count()
    }

    fn dequeue(&mut self) -> Option<(Reindeer, isize)> {
        let result;
        loop {
            match self.candidates.pop() {
                None => return None,
                Some((r, score)) => {
                    let score = -score;
                    if !self.visited.contains_key(&r) {
                        self.visited.insert(r, score);
                        result = Some((r, score));
                        break;
                    }
                }
            }
        }
        result
    }

    fn visited_towards_exit(&self) -> HashSet<Position> {
        let mut result = HashSet::new();
        breadth_first_search(&self.exit, |s, q| {
            result.insert(*s);
            if *s == self.entrance {
                ContinueSearch::No
            } else {
                for parent in self.parents.get_vec(s).unwrap() {
                    q.enqueue(parent);
                }
                ContinueSearch::Yes
            }
        });
        result
    }
}

impl Iterator for ReindeerPathTable {
    type Item = (Reindeer, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.dequeue();
        if let Some((parent, parent_score)) = result {
            for (candidate, candidate_score) in parent.futures(parent_score) {
                if self.maze.value(candidate.p).unwrap() != '#' {
                    self.parents.insert(candidate.p, parent.p);
                    let new_priority = -candidate_score;
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
