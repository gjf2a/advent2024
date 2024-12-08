use std::collections::{HashMap, HashSet};

use advent2024::{chooser_main, grid::GridCharWorld};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let world = GridCharWorld::from_char_file(filename)?;
        let mut antenna2locations = HashMap::new();
        for (p, v) in world.position_value_iter() {
            match antenna2locations.get_mut(v) {
                None => {antenna2locations.insert(*v, vec![*p]);}
                Some(ps) => {ps.push(*p);}
            }
        }

        let mut antinodes = HashSet::new();
        for (_, locations) in antenna2locations.iter() {
            for i in 0..locations.len() {
                for j in (i + 1)..locations.len() {
                    let x_diff = (locations[i][0] - locations[j][0]).abs();
                    let y_diff = (locations[i][1] - locations[j][1]).abs();
                }
            }
        }
        println!("{}", antinodes.len());
        Ok(())
    })
}

