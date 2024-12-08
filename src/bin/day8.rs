use std::collections::{HashMap, HashSet};

use advent2024::{advent_main, grid::GridCharWorld, multidim::Position, Part};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let map = Antennae::new(filename, part)?;
        println!("{}", map.antinodes.len());
        Ok(())
    })
}

struct Antennae {
    world: GridCharWorld,
    antenna2locations: HashMap<char, Vec<Position>>,
    antinodes: HashSet<Position>,
}

impl Antennae {
    fn new(filename: &str, part: Part) -> anyhow::Result<Self> {
        let world = GridCharWorld::from_char_file(filename)?;
        let antenna2locations = antenna_map(&world);
        let mut ant = Self {
            world,
            antenna2locations,
            antinodes: HashSet::new(),
        };
        ant.find_antinodes(part);
        Ok(ant)
    }

    fn find_antinodes(&mut self, part: Part) {
        for (_, locations) in self.antenna2locations.clone().iter() {
            for i in 0..locations.len() {
                for j in (i + 1)..locations.len() {
                    let diff = locations[i] - locations[j];
                    self.add_antinodes(locations[i], diff, part);
                    self.add_antinodes(locations[j], -diff, part);
                }
            }
        }
    }

    fn add_antinodes(&mut self, mut candidate: Position, diff: Position, part: Part) {
        match part {
            Part::One => {
                candidate += diff;
                if self.world.in_bounds(candidate) {
                    self.antinodes.insert(candidate);
                }
            }
            Part::Two => {
                while self.world.in_bounds(candidate) {
                    self.antinodes.insert(candidate);
                    candidate += diff;
                }
            }
        }
    }
}

fn antenna_map(world: &GridCharWorld) -> HashMap<char, Vec<Position>> {
    let mut antenna2locations = HashMap::new();
    for (p, v) in world.position_value_iter() {
        if *v != '.' {
            match antenna2locations.get_mut(v) {
                None => {
                    antenna2locations.insert(*v, vec![*p]);
                }
                Some(ps) => {
                    ps.push(*p);
                }
            }
        }
    }
    antenna2locations
}
