use advent2024::advent_main;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        println!("{filename} {part:?}");
        Ok(())
    })
}
