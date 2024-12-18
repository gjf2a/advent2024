use std::collections::{BTreeMap, BTreeSet, VecDeque};

use advent2024::{
    advent_main, all_lines, grid::GridCharWorld, multidim::{DirType, ManhattanDir, Position}, Part
};
use common_macros::b_tree_map;
use enum_iterator::all;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let (dim, falls) = if filename.contains("ex") { (7, 12) } else { (71, 1024) };
        let goal = Position::from((dim, dim));
        let bombs = all_lines(filename)?
            .map(|line| line.parse::<Position>().unwrap())
            .collect();
        let reachable = ReachableSquares::new(bombs, dim, options.contains(&"-view"));
        println!("number of bombs: {}", reachable.bombs.len());
        match part {
            Part::One => {
                let after1024 = reachable.skip(falls - 1).next().unwrap();
                println!("skipped");
                let closest = after1024
                    .iter()
                    .map(|(r, d)| goal.manhattan_distance(r) + d)
                    .min()
                    .unwrap();
                println!("{closest}");
            }
            Part::Two => {
                todo!()
            }
        }
        Ok(())
    })
}

struct ReachableSquares {
    reachable: Option<BTreeMap<Position, isize>>,
    bombs: VecDeque<Position>,
    fallen_bombs: BTreeSet<Position>,
    dim: isize,
    view: bool,
}

impl ReachableSquares {
    fn new(bombs: VecDeque<Position>, dim: isize, view: bool) -> Self {
        Self {
            bombs,
            reachable: Some(b_tree_map! {Position::from((0, 0)) => 0}),
            fallen_bombs: BTreeSet::new(),
            dim,
            view,
        }
    }

    fn in_bounds(&self, candidate: Position) -> bool {
        candidate.values().all(|v| v >= 0 && v <= self.dim)
    }

    fn view(&self) {
        if self.view {
            let mut grid = GridCharWorld::new(self.dim as usize, self.dim as usize, '.');
            for f in self.fallen_bombs.iter() {
                grid.update(*f, '#');
            }
            for (r, _) in self.reachable.as_ref().unwrap().iter() {
                grid.update(*r, 'R');
            }
            println!("{}\n{grid}\n", self.fallen_bombs.len());
        }
    }
}

impl Iterator for ReachableSquares {
    type Item = BTreeMap<Position, isize>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.reachable.clone();
        self.reachable = match self.bombs.pop_front() {
            None => None,
            Some(bomb) => {
                self.fallen_bombs.insert(bomb);
                let mut newly_reachable = BTreeMap::new();
                for (r, dist) in self.reachable.as_ref().unwrap().iter() {
                    for dir in all::<ManhattanDir>() {
                        let candidate = dir.neighbor(*r);
                        if self.in_bounds(candidate) && !self.fallen_bombs.contains(&candidate) {
                            newly_reachable.insert(candidate, dist + 1);
                        }
                    }
                }
                self.view();
                Some(newly_reachable)
            }
        };
        result
    }
}
