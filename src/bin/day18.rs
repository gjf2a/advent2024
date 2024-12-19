use std::collections::{BTreeMap, BTreeSet, VecDeque};

use advent2024::{
    advent_main, all_lines, grid::GridCharWorld, multidim::{DirType, ManhattanDir, Position}, Part
};
use common_macros::b_tree_map;
use enum_iterator::all;
use pancurses::{endwin, initscr, noecho, Input};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let (dim, falls) = if filename.contains("ex") { (7, 12) } else { (71, 1024) };
        let goal = Position::from((dim, dim));
        let bombs = all_lines(filename)?
            .map(|line| line.parse::<Position>().unwrap())
            .collect();
        let reachable = ReachableSquares::new(bombs, dim);
        if options.contains(&"-view") {
            let window = initscr();
            window.keypad(true);
            noecho();
            for (step, (_, fallen_bombs)) in reachable.enumerate() {
                let grid = bomb_grid_from(dim as usize, &fallen_bombs);
                window.clear();
                window.addstr(format!("Step {step}:\n{grid}"));
                match window.getch() {
                    Some(Input::Character(c)) => match c {
                        'q' => break,
                        _ => {}
                    }
                    Some(Input::KeyDC) => break,
                    _ => {}
                }
            }
            endwin();
        } else {
            match part {
                Part::One => part1(reachable, falls, goal),
                Part::Two => {
                    todo!()
                }
            }
        }
        Ok(())
    })
}

fn part1(reachable: ReachableSquares, falls: usize, goal: Position) {
    let (reachable, _) = reachable.skip(falls - 1).next().unwrap();
    println!("skipped");
    let closest = reachable
        .iter()
        .map(|(r, d)| goal.manhattan_distance(r) + d)
        .min()
        .unwrap();
    println!("{closest}");
}

struct ReachableSquares {
    reachable: BTreeMap<Position, isize>,
    bombs: VecDeque<Position>,
    fallen_bombs: BTreeSet<Position>,
    dim: isize,
}

impl ReachableSquares {
    fn new(bombs: VecDeque<Position>, dim: isize) -> Self {
        Self {
            bombs,
            reachable: b_tree_map! {Position::from((0, 0)) => 0},
            fallen_bombs: BTreeSet::new(),
            dim,
        }
    }

    fn in_bounds(&self, candidate: Position) -> bool {
        candidate.values().all(|v| v >= 0 && v <= self.dim)
    }
}

fn bomb_grid_from(dim: usize, fallen_bombs: &BTreeSet<Position>) -> GridCharWorld {
    let mut grid = GridCharWorld::new(dim, dim, '.');
    for f in fallen_bombs.iter() {
        grid.update(*f, '#');
    }
    grid
}

fn grid_from(dim: usize, reachable: &BTreeMap<Position, isize>, fallen_bombs: &BTreeSet<Position>) -> GridCharWorld {
    let mut grid = GridCharWorld::new(dim, dim, '.');
    for f in fallen_bombs.iter() {
        grid.update(*f, '#');
    }
    for (r, _) in reachable.iter() {
        grid.update(*r, 'R');
    }
    grid
}

impl Iterator for ReachableSquares {
    type Item = (BTreeMap<Position, isize>, BTreeSet<Position>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.bombs.pop_front() {
            None => None,
            Some(bomb) => {
                let result = (self.reachable.clone(), self.fallen_bombs.clone());
                self.fallen_bombs.insert(bomb);
                let mut newly_reachable = BTreeMap::new();
                for (r, dist) in self.reachable.iter() {
                    for dir in all::<ManhattanDir>() {
                        let candidate = dir.neighbor(*r);
                        if self.in_bounds(candidate) && !self.fallen_bombs.contains(&candidate) {
                            newly_reachable.insert(candidate, dist + 1);
                        }
                    }
                }
                std::mem::swap(&mut self.reachable, &mut newly_reachable);
                Some(result)
            }
        }
    }
}
