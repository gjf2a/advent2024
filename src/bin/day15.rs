use advent2024::{
    advent_main, all_lines,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position}, Part,
};
use itertools::Itertools;
use std::{collections::VecDeque, fmt::Display};
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let world: &mut dyn RobotWorld = match part {
            Part::One => &mut RobotWorld1::new(filename)?,
            Part::Two => &mut RobotWorld2::new(filename)?,
        };
        println!("{world}");
        while !world.done() {
            world.advance();
        }
        println!("{}", world.gps_sum());
        Ok(())
    })
}

trait RobotWorld: Display {
    fn done(&self) -> bool;
    fn advance(&mut self);
    fn gps_sum(&self) -> isize;
}

struct RobotWorld1 {
    grid: GridCharWorld,
    robot: Position,
    script: VecDeque<ManhattanDir>,
}

impl RobotWorld1 {
    fn new(filename: &str) -> anyhow::Result<Self> {
        let mut grid_chars = String::new();
        let mut move_chars = String::new();
        let mut in_grid = true;
        for line in all_lines(filename)? {
            if line.len() == 0 {
                in_grid = false;
            } else if in_grid {
                grid_chars.push_str(line.as_str());
                grid_chars.push('\n');
            } else {
                move_chars.push_str(line.as_str());
            }
        }

        let grid = grid_chars.parse::<GridCharWorld>()?;
        let robot = find_robot(&grid);
        let script = parse_moves(move_chars.as_str());
        Ok(Self {
            grid,
            robot,
            script,
        })
    }
}

impl Display for RobotWorld1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{:?}\n", self.grid, self.script)
    }
}

impl RobotWorld for RobotWorld1 {
    fn done(&self) -> bool {
        self.script.is_empty()
    }

    fn advance(&mut self) {
        if let Some(dir) = self.script.pop_front() {
            let ray = dir
                .iter_from(self.robot)
                .map(|p| (p, self.grid.value(p).unwrap()))
                .take_while_inclusive(|(_, c)| *c != '#' && *c != '.')
                .collect::<Vec<_>>();
            if ray.last().unwrap().1 == '.' {
                for i in (0..(ray.len() - 1)).rev() {
                    self.grid.update(ray[i + 1].0, ray[i].1);
                }
                self.grid.update(self.robot, '.');
                self.robot = ray[1].0;
            }
        }
    }

    fn gps_sum(&self) -> isize {
        self.grid
            .position_value_iter()
            .filter(|(_, v)| **v == 'O')
            .map(|(p, _)| p[0] + 100 * p[1])
            .sum()
    }
}

struct RobotWorld2 {
    grid: GridCharWorld,
    robot: Position,
    script: VecDeque<ManhattanDir>,
}

impl RobotWorld2 {
    fn new(filename: &str) -> anyhow::Result<Self> {
        let mut grid_chars = String::new();
        let mut move_chars = String::new();
        let mut in_grid = true;
        for line in all_lines(filename)? {
            if line.len() == 0 {
                in_grid = false;
            } else if in_grid {
                let mut wide_line = String::new();
                for c in line.chars() {
                    let widened = match c {
                        '.' => "..",
                        '#' => "##",
                        'O' => "[]",
                        '@' => "@.",
                        _ => return Err(anyhow!("Unrecognized char: {c}"))
                    };
                    wide_line.push_str(widened);
                }
                grid_chars.push_str(wide_line.as_str());
                grid_chars.push('\n');
            } else {
                move_chars.push_str(line.as_str());
            }
        }

        let grid = grid_chars.parse::<GridCharWorld>()?;
        let robot = find_robot(&grid);
        let script = parse_moves(move_chars.as_str());
        Ok(Self {
            grid,
            robot,
            script,
        })
    }
}

impl RobotWorld for RobotWorld2 {
    fn gps_sum(&self) -> isize {
        todo!("Implement tricky rules for boxes");
    }

    fn done(&self) -> bool {
        self.script.is_empty()
    }

    fn advance(&mut self) {
        if let Some(dir) = self.script.pop_front() {
            todo!("Handle tricky boxes");
        }
    }
}

impl Display for RobotWorld2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{:?}\n", self.grid, self.script)
    }
}

fn parse_moves(move_chars: &str) -> VecDeque<ManhattanDir> {
    move_chars
        .chars()
        .map(|c| match c {
            '^' => ManhattanDir::N,
            'v' => ManhattanDir::S,
            '<' => ManhattanDir::W,
            '>' => ManhattanDir::E,
            _ => panic!("Unrecognized: {c}"),
        })
        .collect()
}

fn find_robot(grid: &GridCharWorld) -> Position {
    grid
    .position_value_iter()
    .find(|(_, v)| **v == '@')
    .map(|(p, _)| *p)
    .unwrap()
}