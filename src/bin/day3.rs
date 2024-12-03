use advent2024::{all_lines, chooser_main, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        for (i, line) in all_lines(filename)?.enumerate() {
            println!("{i}: {line}");
        }
        Ok(())
    })
}