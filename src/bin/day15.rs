use advent2024::{
    advent_main, all_lines,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
};
use itertools::Itertools;
use std::collections::VecDeque;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let mut world = RobotWorld::new(filename)?;
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
}

impl RobotWorld {
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
        let robot = grid
            .position_value_iter()
            .find(|(_, v)| **v == '@')
            .map(|(p, _)| *p)
            .unwrap();
        let script = move_chars
            .chars()
            .map(|c| match c {
                '^' => ManhattanDir::N,
                'v' => ManhattanDir::S,
                '<' => ManhattanDir::W,
                '>' => ManhattanDir::E,
                _ => panic!("Unrecognized: {c}"),
            })
            .collect();
        Ok(Self {
            grid,
            robot,
            script,
        })
    }

    fn gps_sum(&self) -> isize {
        self.grid
            .position_value_iter()
            .filter(|(_, v)| **v == 'O')
            .map(|(p, _)| p[0] + 100 * p[1])
            .sum()
    }

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
}
