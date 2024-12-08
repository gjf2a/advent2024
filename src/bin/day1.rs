use advent2024::{all_lines, advent_main, Part};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        match part {
            Part::One => {
                let (mut col1, mut col2) = get_lists(filename)?;
                col1.sort();
                col2.sort();
                let total_diff: i64 = col1
                    .iter()
                    .zip(col2.iter())
                    .map(|(a, b)| (a - b).abs())
                    .sum();
                println!("{total_diff}");
            }
            Part::Two => {
                let (col1, col2) = get_lists(filename)?;
                let similarity: i64 = col1
                    .iter()
                    .map(|a| col2.iter().filter(|b| a == *b).count() as i64 * a)
                    .sum();
                println!("{similarity}");
            }
        }

        Ok(())
    })
}

fn get_lists(filename: &str) -> anyhow::Result<(Vec<i64>, Vec<i64>)> {
    let mut col1 = vec![];
    let mut col2 = vec![];
    for line in all_lines(filename)? {
        let parts = line
            .split_whitespace()
            .map(|p| p.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        col1.push(parts[0]);
        col2.push(parts[1]);
    }
    Ok((col1, col2))
}
