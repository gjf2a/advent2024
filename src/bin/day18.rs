use std::collections::{BTreeSet, HashMap};

use advent2024::{
    advent_main, all_lines,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    search_iter::BfsIter,
    Part,
};
use enum_iterator::all;
use pancurses::{endwin, initscr, noecho, Input};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let (dim, falls) = if filename.contains("ex") {
            (7, 12)
        } else {
            (71, 1024)
        };
        let goal = Position::from((dim - 1, dim - 1));
        let bombs = all_lines(filename)?
            .map(|line| line.parse::<Position>().unwrap())
            .collect::<Vec<_>>();
        if options.contains(&"-view") {
            view(dim, &bombs);
        } else {
            match part {
                Part::One => part1(bombs, falls, dim, goal),
                Part::Two => part2(bombs, dim, goal),
            }
        }
        Ok(())
    })
}

fn part1(bombs: Vec<Position>, falls: usize, dim: isize, goal: Position) {
    let fallen_bombs = (&bombs[0..falls]).iter().copied().collect::<BTreeSet<_>>();
    let exit = find_exit(&fallen_bombs, goal, dim);
    println!("{exit:?}");
}

fn part2(bombs: Vec<Position>, dim: isize, goal: Position) {
    let result = find_impassible(&bombs, dim, goal);
    println!("{result}");
}

fn find_impassible(bombs: &Vec<Position>, dim: isize, goal: Position) -> Position {
    let mut fallen_bombs = BTreeSet::new();
    for b in bombs.iter() {
        fallen_bombs.insert(*b);
        if find_exit(&fallen_bombs, goal, dim).is_none() {
            return *b;
        }
    }
    todo!("Impossible");
}

fn find_exit(fallen_bombs: &BTreeSet<Position>, goal: Position, dim: isize) -> Option<usize> {
    let start = Position::default();
    let mut distances = HashMap::<Position, usize>::new();
    distances.insert(start, 0);
    BfsIter::new(start, |point| {
        all::<ManhattanDir>()
            .map(|d| d.neighbor(*point))
            .filter(|neighbor| {
                !fallen_bombs.contains(neighbor)
                    && neighbor.values().all(|v| v >= 0 && v < dim as isize)
            })
            .map(|neighbor| {
                let parent_distance = distances.get(point).copied().unwrap();
                let current_distance = parent_distance + 1;
                let prev_distance = distances.get(&neighbor);
                if prev_distance.is_none() || current_distance < prev_distance.copied().unwrap() {
                    distances.insert(neighbor, current_distance);
                }
                neighbor
            })
            .collect()
    })
    .find(|p| *p == goal)
    .map(|p| distances.get(&p).copied().unwrap())
}

fn view(dim: isize, bombs: &Vec<Position>) {
    let window = initscr();
    window.keypad(true);
    noecho();
    let mut grid = GridCharWorld::new(dim as usize, dim as usize, '.');
    for (step, bomb) in bombs.iter().enumerate() {
        window.clear();
        window.addstr(format!("Step {step}:\n{grid}"));
        grid.update(*bomb, '#');
        match window.getch() {
            Some(Input::Character(c)) => match c {
                'q' => break,
                _ => {}
            },
            Some(Input::KeyDC) => break,
            _ => {}
        }
    }
    endwin();
}
