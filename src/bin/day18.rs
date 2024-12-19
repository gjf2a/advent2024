use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

use advent2024::{
    advent_main, all_lines, grid::GridCharWorld, multidim::{DirType, ManhattanDir, Position}, searchers::{breadth_first_search, ContinueSearch, SearchQueue}, Part
};
use common_macros::b_tree_map;
use enum_iterator::all;
use pancurses::{endwin, initscr, noecho, Input};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let (dim, falls) = if filename.contains("ex") { (7, 12) } else { (71, 1024) };
        let goal = Position::from((dim - 1, dim - 1));
        let bombs = all_lines(filename)?
            .map(|line| line.parse::<Position>().unwrap())
            .collect();
        let reachable = BombedSquares::new(bombs, dim);
        if options.contains(&"-view") {
            let window = initscr();
            window.keypad(true);
            noecho();
            for (step, fallen_bombs) in reachable.enumerate() {
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

fn part1(reachable: BombedSquares, falls: usize, goal: Position) {
    let dim = reachable.dim as usize;
    let fallen_bombs = reachable.skip(falls).next().unwrap();
    let mut bomb_grid = bomb_grid_from(dim, &fallen_bombs);
    println!("{bomb_grid}");
    let mut closest = None;
    let start = Position::default();
    let mut distances = HashMap::<Position, usize>::new();
    distances.insert(start, 0);
    let q = breadth_first_search(&start, |point, q| {
        if *point == goal {
            closest = Some(*point);
            ContinueSearch::No
        } else {
            for dir in all::<ManhattanDir>() {
                let neighbor = dir.neighbor(*point);
                if !fallen_bombs.contains(&neighbor) && neighbor.values().all(|v| v >= 0 && v < dim as isize) {
                    let parent_distance = distances.get(point).copied().unwrap();
                    let current_distance = parent_distance + 1;
                    let prev_distance = distances.get(&neighbor);
                    if prev_distance.is_none() || current_distance < prev_distance.copied().unwrap() {
                        distances.insert(neighbor, current_distance);
                    }
                    q.enqueue(&neighbor);
                }
            }
            ContinueSearch::Yes
        }
    });
    for p in q.path_back_from(closest.as_ref().unwrap()).unwrap() {
        bomb_grid.update(p, 'O');
    }
    println!();
    println!("{bomb_grid}");
    println!("{closest:?}");
    println!("{:?}", distances.get(&closest.unwrap()));
}

struct BombedSquares {
    bombs: VecDeque<Position>,
    fallen_bombs: BTreeSet<Position>,
    dim: isize,
}

impl BombedSquares {
    fn new(bombs: VecDeque<Position>, dim: isize) -> Self {
        Self {
            bombs,
            fallen_bombs: BTreeSet::new(),
            dim,
        }
    }
}

fn bomb_grid_from(dim: usize, fallen_bombs: &BTreeSet<Position>) -> GridCharWorld {
    let mut grid = GridCharWorld::new(dim, dim, '.');
    for f in fallen_bombs.iter() {
        grid.update(*f, '#');
    }
    grid
}

impl Iterator for BombedSquares {
    type Item = BTreeSet<Position>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.bombs.pop_front() {
            None => None,
            Some(bomb) => {
                let result = self.fallen_bombs.clone();
                self.fallen_bombs.insert(bomb);
                Some(result)
            }
        }
    }
}
