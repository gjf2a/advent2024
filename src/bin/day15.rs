use advent2024::{
    advent_main, all_lines,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    Part,
};
use anyhow::anyhow;
use indexmap::IndexSet;
use itertools::Itertools;
use pancurses::{endwin, initscr, noecho, Input};
use std::{collections::VecDeque, fmt::Display};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let mut world = RobotWorld::new(filename, part)?;
        if options.contains(&"-show") {
            println!("{world}");
        }
        if options.contains(&"-visualize") {
            visualize(&mut world);
        } else {
            while !world.done() {
                world.advance();
            }
        }
        println!("{}", world.gps_sum());
        Ok(())
    })
}

struct RobotWorld {
    grid: GridCharWorld,
    robot: Position,
    script: VecDeque<ManhattanDir>,
    part: Part,
}

impl Display for RobotWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{:?}\n", self.grid, self.script)
    }
}

impl RobotWorld {
    fn new(filename: &str, part: Part) -> anyhow::Result<Self> {
        let mut grid_chars = String::new();
        let mut move_chars = String::new();
        let mut in_grid = true;
        for line in all_lines(filename)? {
            if line.len() == 0 {
                in_grid = false;
            } else if in_grid {
                let line = match part {
                    Part::One => line,
                    Part::Two => widened_line(line.as_str())?,
                };
                grid_chars.push_str(line.as_str());
                grid_chars.push('\n');
            } else {
                move_chars.push_str(line.as_str());
            }
        }

        let grid = grid_chars.parse::<GridCharWorld>()?;
        let robot = grid.any_position_for('@');
        let script = parse_moves(move_chars.as_str());
        Ok(Self {
            grid,
            robot,
            script,
            part,
        })
    }

    fn gps_sum(&self) -> isize {
        self.grid
            .position_value_iter()
            .filter(|(_, v)| "O[".contains(**v))
            .map(|(p, _)| p[0] + 100 * p[1])
            .sum()
    }

    fn done(&self) -> bool {
        self.script.is_empty()
    }

    fn advance(&mut self) {
        if let Some(dir) = self.script.pop_front() {
            if self.part == Part::One || [ManhattanDir::E, ManhattanDir::W].contains(&dir) {
                self.advance_narrow(dir);
            } else {
                self.advance_wide(dir);
            }
        }
    }
    
    fn advance_narrow(&mut self, dir: ManhattanDir) {
        let ray = dir
            .iter_from(self.robot)
            .take_while_inclusive(|f| self.grid.value(*f).map_or(false, |c| !".#".contains(c)))
            .collect::<Vec<_>>();
        if self.grid.value(*ray.last().unwrap()).unwrap() == '.' {
            for i in (0..(ray.len() - 1)).rev() {
                self.grid.swap(ray[i + 1], ray[i]);
            }
            self.robot = ray[1];
        }
    }

    fn advance_wide(&mut self, dir: ManhattanDir) {
        if let Some(move_points) = self.fan_ray(dir) {
            for i in (0..move_points.len()).rev() {
                for p in move_points[i].iter() {
                    self.grid.swap(*p, dir.neighbor(*p));
                    if i == 0 {
                        assert_eq!(1, move_points[i].len());
                        self.robot = dir.neighbor(*p);
                    }
                }
            }
        }
    }

    fn fan_ray(&self, dir: ManhattanDir) -> Option<Vec<IndexSet<Position>>> {
        let mut start = IndexSet::new();
        start.insert(self.robot);
        let mut fringes = vec![start];
        let mut all_space = false;
        while !all_space {
            all_space = true;
            let last_fringe = fringes.last().unwrap();
            let next_points = last_fringe
                .iter()
                .map(|p| dir.neighbor(*p))
                .collect::<Vec<_>>();
            let next_values = next_points
                .iter()
                .map(|p| self.grid.value(*p).unwrap())
                .collect::<Vec<_>>();
            let mut fringe = IndexSet::new();
            for i in 0..next_values.len() {
                match next_values[i] {
                    '#' => return None,
                    '.' => {}
                    '[' => {
                        all_space = false;
                        fringe.insert(next_points[i]);
                        fringe.insert(ManhattanDir::E.neighbor(next_points[i]));
                    }
                    ']' => {
                        all_space = false;
                        fringe.insert(ManhattanDir::W.neighbor(next_points[i]));
                        fringe.insert(next_points[i]);
                    }
                    _ => panic!("Unrecognized character: {}", next_values[i]),
                }
            }
            fringes.push(fringe);
        }
        Some(fringes)
    }
}

fn widened_line(line: &str) -> anyhow::Result<String> {
    let mut wide_line = String::new();
    for c in line.chars() {
        let widened = match c {
            '.' => "..",
            '#' => "##",
            'O' => "[]",
            '@' => "@.",
            _ => return Err(anyhow!("Unrecognized char: {c}")),
        };
        wide_line.push_str(widened);
    }
    Ok(wide_line)
}

fn parse_moves(move_chars: &str) -> VecDeque<ManhattanDir> {
    move_chars
        .chars()
        .map(ManhattanDir::try_from)
        .map(|d| d.unwrap())
        .collect()
}

fn visualize(world: &mut RobotWorld) {
    let window = initscr();
    window.keypad(true);
    noecho();

    loop {
        window.clear();
        let message = if world.done() {
            "Finished".to_owned()
        } else {
            format!(
                "Next move: {:?} ({} left)\n",
                world.script[0],
                world.script.len()
            )
        };
        window.addstr(message);
        window.addstr(format!("{}", world.grid));
        match window.getch() {
            Some(Input::Character(c)) => match c {
                ' ' => world.advance(),
                'q' => break,
                _ => {}
            },
            Some(Input::KeyDC) => break,
            _ => (),
        }
    }

    endwin();
}
