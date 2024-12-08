use std::collections::{HashMap, HashSet};

use advent2024::{chooser_main, grid::GridCharWorld, multidim::Position, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let world = GridCharWorld::from_char_file(filename)?;
        let antenna2locations = antenna_map(&world);
        println!("{}", find_antinodes(part, &antenna2locations, &world).len());
        Ok(())
    })
}

fn find_antinodes(
    part: Part,
    antenna2locations: &HashMap<char, Vec<Position>>,
    world: &GridCharWorld,
) -> HashSet<Position> {
    let mut antinodes = HashSet::new();
    for (_, locations) in antenna2locations.iter() {
        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                let diff = locations[i] - locations[j];
                match part {
                    Part::One => {
                        add_antinode(&mut antinodes, locations[i] + diff, &world);
                        add_antinode(&mut antinodes, locations[j] - diff, &world);
                    }
                    Part::Two => {
                        add_antinode_streak(&mut antinodes, locations[i], &world, diff);
                        add_antinode_streak(&mut antinodes, locations[j], &world, -diff);
                    }
                }
            }
        }
    }
    antinodes
}

fn add_antinode(antinodes: &mut HashSet<Position>, candidate: Position, world: &GridCharWorld) {
    if world.in_bounds(candidate) {
        antinodes.insert(candidate);
    }
}

fn add_antinode_streak(
    antinodes: &mut HashSet<Position>,
    mut candidate: Position,
    world: &GridCharWorld,
    diff: Position,
) {
    while world.in_bounds(candidate) {
        antinodes.insert(candidate);
        candidate += diff;
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
