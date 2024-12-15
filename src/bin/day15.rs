use advent2024::{
    advent_main, all_lines,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position}, Part,
};
use itertools::Itertools;
use std::{collections::VecDeque, fmt::Display};
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let mut world = RobotWorld::new(filename, part)?;
        if options.contains(&"-show") {
            println!("{world}");
        }
        while !world.done() {
            world.advance();
        }
        println!("{}", world.gps_sum());
        Ok(())
    })
}

struct RobotWorld {
    grid: GridCharWorld,
    robot: Position,
    script: VecDeque<ManhattanDir>,
    part: Part
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
                    Part::Two => {
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
                        wide_line
                    }
                };
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
            part
        })
    } 

    fn done(&self) -> bool {
        self.script.is_empty()
    }

    fn advance(&mut self) {
        if let Some(dir) = self.script.pop_front() {
            if self.part == Part::One || [ManhattanDir::E, ManhattanDir::W].contains(&dir) {
                let ray = dir
                    .iter_from(self.robot)
                    .map(|p| (p, self.grid.value(p).unwrap()))
                    .take_while_inclusive(|(_, c)| !".#".contains(*c))
                    .collect::<Vec<_>>();
                if ray.last().unwrap().1 == '.' {
                    for i in (0..(ray.len() - 1)).rev() {
                        self.grid.swap(ray[i + 1].0, ray[i].0);
                    }
                    self.robot = ray[1].0;
                }
            } else {
                todo!("Handle the wide boxes")
            }
        }
    }

    fn gps_sum(&self) -> isize {
        self.grid
            .position_value_iter()
            .filter(|(_, v)| "O[".contains(**v))
            .map(|(p, _)| p[0] + 100 * p[1])
            .sum()
    }
}

fn parse_moves(move_chars: &str) -> VecDeque<ManhattanDir> {
    move_chars
        .chars()
        .map(ManhattanDir::try_from)
        .map(|d| d.unwrap())
        .collect()
}

fn find_robot(grid: &GridCharWorld) -> Position {
    grid
    .position_value_iter()
    .find(|(_, v)| **v == '@')
    .map(|(p, _)| *p)
    .unwrap()
}