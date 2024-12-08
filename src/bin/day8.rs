use std::collections::{HashMap, HashSet};

use advent2024::{chooser_main, grid::GridCharWorld, multidim::Position};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let world = GridCharWorld::from_char_file(filename)?;
        let mut antenna2locations = HashMap::new();
        for (p, v) in world.position_value_iter() {
            if *v != '.' {
                match antenna2locations.get_mut(v) {
                    None => {antenna2locations.insert(*v, vec![*p]);}
                    Some(ps) => {ps.push(*p);}
                }
            }
        }

        let mut antinodes = HashSet::new();
        for (ant, locations) in antenna2locations.iter() {
            for i in 0..locations.len() {
                for j in (i + 1)..locations.len() {
                    let diff = locations[i] - locations[j];
                    add_antinode(&mut antinodes, locations[i] + diff, &world);
                    add_antinode(&mut antinodes, locations[j] - diff, &world);
                }
            }
        }
        println!("{}", antinodes.len());
        Ok(())
    })
}

fn add_antinode(antinodes: &mut HashSet<Position>, candidate: Position, world: &GridCharWorld) {
    if world.in_bounds(candidate) {
        antinodes.insert(candidate);
    }
}