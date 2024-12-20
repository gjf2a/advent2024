use std::collections::HashMap;

use advent2024::{
    advent_main,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    Part,
};
use enum_iterator::all;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let cheat_min = find_cheat_min(options);
        let maze = GridCharWorld::from_char_file(filename)?;
        let distances = Distances::new(&maze);
        let cheat_dist = match part {
            Part::One => 2,
            Part::Two => 20,
        };
        let good_cheats = distances.incoming_cheat_count(cheat_dist, cheat_min);
        println!("{good_cheats}");
        Ok(())
    })
}

#[derive(Clone)]
struct Distances {
    no_cheat: usize,
    dist2end: HashMap<Position, usize>,
    dist2start: HashMap<Position, usize>,
    maze: GridCharWorld,
}

impl Distances {
    fn new(maze: &GridCharWorld) -> Self {
        let (no_cheat, dist2end) = step_map(&maze, 'E', 'S');
        let (repeat, dist2start) = step_map(&maze, 'S', 'E');
        assert_eq!(no_cheat, repeat);
        let maze = maze.clone();
        Self {
            no_cheat,
            dist2end,
            dist2start,
            maze,
        }
    }

    fn incoming_cheat_count(&self, cheat_dist: usize, cheat_min: usize) -> usize {
        self.maze
            .position_iter()
            .map(|p| {
                self.incoming_cheat_savings(p, cheat_dist)
                    .iter()
                    .filter(|s| **s >= cheat_min)
                    .count()
            })
            .sum::<usize>()
    }

    fn incoming_cheat_savings(&self, p: Position, cheat_dist: usize) -> Vec<usize> {
        let mut result = vec![];
        if let Some(to_end) = self.dist2end.get(&p) {
            if let Some(to_start) = self.dist2start.get(&p) {
                for candidate in self.maze.position_iter() {
                    let dist = candidate.manhattan_distance(&p) as usize;
                    if dist <= cheat_dist {
                        if let Some(candidate_start) = self.dist2start.get(&candidate) {
                            let cheat_path = *to_end + dist + candidate_start;
                            if cheat_path < to_start + to_end {
                                result.push(self.no_cheat - cheat_path);
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

fn step_map(
    maze: &GridCharWorld,
    start_char: char,
    end_char: char,
) -> (usize, HashMap<Position, usize>) {
    let mut dist2end = HashMap::new();
    let mut current = maze.any_position_for(start_char);
    let mut dist = 0;
    loop {
        dist2end.insert(current, dist);
        if maze.value(current).unwrap() == end_char {
            return (dist, dist2end);
        }
        dist += 1;
        current = all::<ManhattanDir>()
            .map(|d| d.neighbor(current))
            .find(|p| maze.value(*p).unwrap() != '#' && !dist2end.contains_key(p))
            .unwrap();
    }
}

fn find_cheat_min(options: Vec<&str>) -> usize {
    for opt in options {
        if opt.starts_with("-min") {
            return opt.split(":").skip(1).next().unwrap().parse().unwrap();
        }
    }
    100
}
