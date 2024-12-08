use advent2024::{advent_main, all_lines, Part};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let mul_regex = regex::Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)")?;
        let param_regex = regex::Regex::new(r"\d+")?;
        let mut total = 0;
        let mut enabled = true;
        for line in all_lines(filename)? {
            for m in mul_regex.find_iter(line.as_str()) {
                if enabled && m.as_str().starts_with("mul") {
                    total += param_regex
                        .find_iter(m.as_str())
                        .map(|s| s.as_str().parse::<i64>().unwrap())
                        .product::<i64>();
                } else if part == Part::Two {
                    enabled = m.as_str() == "do()";
                }
            }
        }
        println!("{total}");
        Ok(())
    })
}
