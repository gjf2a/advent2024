use advent2024::chooser_main;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        println!("{filename} {part:?}");
        Ok(())
    })
}

