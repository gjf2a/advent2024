use std::collections::{HashMap, HashSet};

use advent2024::{
    advent_main,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    searchers::{breadth_first_search, ContinueSearch, SearchQueue},
    Part,
};
use enum_iterator::all;
use multimap::MultiMap;
use priority_queue::PriorityQueue;

const MOVE_COST: isize = 1;
const TURN_COST: isize = 1000;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let table = ReindeerPathTable::new(GridCharWorld::from_char_file(filename)?);
        match part {
            Part::One => part1(table),
            Part::Two => part2(table, options.contains(&"-show")),
        }
        Ok(())
    })
}

fn part1(mut table: ReindeerPathTable) {
    let exit = table.exit;
    let (winner, score) = table
        .by_ref()
        .skip_while(|(r, _)| r.p != exit)
        .next()
        .unwrap();
    assert_eq!(winner.p, exit);
    println!("{}", score);
}

fn part2(mut table: ReindeerPathTable, show: bool) {
    while let Some(_) = table.next() {}
    assert_eq!(table.num_empty_squares(), table.parents.keys().count());
    let on_path = table.visited_towards_exit();
    if show {
        let mut maze = table.maze.clone();
        for p in on_path.iter() {
            maze.update(*p, 'O');
        }
        println!("{maze}");
    }
    println!("{}", on_path.len());
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
        candidates.push(Reindeer::new(entrance, ManhattanDir::E), 0);
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
                let outgoing_dirs = all::<ManhattanDir>()
                    .filter(|d| result.contains(&d.neighbor(*s)))
                    .collect::<Vec<_>>();
                let parents = self.parents.get_vec(s).unwrap();
                let parent_costs = parents
                    .iter()
                    .map(|p| {
                        let incoming_dir = ManhattanDir::dir_from_to(*p, *s).unwrap();
                        let r = Reindeer::new(*p, incoming_dir);
                        let mut c = *self.visited.get(&r).unwrap();
                        if *s != self.exit && !outgoing_dirs.contains(&incoming_dir) {
                            c += TURN_COST;
                        }
                        c
                    })
                    .collect::<Vec<_>>();
                let cheapest_parent = parent_costs.iter().min().unwrap();
                for i in 0..parents.len() {
                    if parent_costs[i] == *cheapest_parent {
                        q.enqueue(&parents[i]);
                    }
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
                    if candidate.p != parent.p {
                        self.parents.insert(candidate.p, parent.p);
                    }
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
    fn new(p: Position, f: ManhattanDir) -> Self {
        Self { p, f }
    }

    fn with_facing(&self, f: ManhattanDir) -> Self {
        Self::new(self.p, f)
    }

    fn with_position(&self, p: Position) -> Self {
        Self::new(p, self.f)
    }

    fn futures(&self, score: isize) -> Vec<(Reindeer, isize)> {
        vec![
            (
                self.with_position(self.f.neighbor(self.p)),
                score + MOVE_COST,
            ),
            (self.with_facing(self.f.clockwise()), score + TURN_COST),
            (
                self.with_facing(self.f.counterclockwise()),
                score + TURN_COST,
            ),
        ]
    }
}
