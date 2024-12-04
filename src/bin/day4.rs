use advent2024::{
    chooser_main,
    grid::GridCharWorld,
    multidim::{Dir, DirType},
    Part,
};
use enum_iterator::all;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let world = GridCharWorld::from_char_file(filename)?;
        let count = match part {
            Part::One => part1(world),
            Part::Two => part2(world),
        };
        println!("{count}");
        Ok(())
    })
}

fn part1(world: GridCharWorld) -> usize {
    let target = vec!['X', 'M', 'A', 'S'];
    world
        .position_iter()
        .map(|p| {
            all::<Dir>()
                .map(|d| world.values_from(p, d, target.len()))
                .filter(|streak| *streak == target)
                .count()
        })
        .sum()
}

fn part2(world: GridCharWorld) -> usize {
    let target = vec!['M', 'A', 'S'];
    let diags = vec![Dir::Nw, Dir::Sw, Dir::Ne, Dir::Se];
    world
        .position_value_iter()
        .filter(|(p, c)| **c == 'A' && !world.at_edge(**p))
        .filter(|(p, _)| {
            diags
                .iter()
                .filter(|d| world.values_from(d.neighbor(**p), d.inverse(), target.len()) == target)
                .count()
                == 2
        })
        .count()
}
