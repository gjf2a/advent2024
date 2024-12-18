use advent2024::advent_main;

// Dynamic programming problem:
// * Start at a clean exit
// * You know you have gone (width + height) - 2 time steps

// Alternative
// Create a grid where each occupied space has a time associated with it.
//

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        println!("{filename} {part:?}");
        Ok(())
    })
}
