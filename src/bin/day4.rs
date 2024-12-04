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
    let mut count = 0;
    let target = vec!['X', 'M', 'A', 'S'];
    for (p, c) in world.position_value_iter() {
        if *c == 'X' {
            count += all::<Dir>()
                .map(|d| world.values_from(*p, d, target.len()))
                .filter(|streak| *streak == target)
                .count();
        }
    }
    count
}

fn part2(world: GridCharWorld) -> usize {
    let mut count = 0;
    let target = vec!['M', 'A', 'S'];
    let diags = vec![Dir::Nw, Dir::Sw, Dir::Ne, Dir::Se];
    for (p, c) in world.position_value_iter() {
        if *c == 'A' && !world.at_edge(*p) {
            let matching_diags = diags
                .iter()
                .filter(|d| world.values_from(d.neighbor(*p), d.inverse(), target.len()) == target)
                .count();
            if matching_diags == 2 {
                count += 1;
            }
        }
    }
    count
}
