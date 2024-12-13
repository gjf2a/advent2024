use advent2024::{advent_main, all_lines, multidim::Position};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let mut inputs = vec![];
        for line in all_lines(filename)? {
            if inputs.len() == 3 {
                println!("{inputs:?}");
                inputs = vec![];
            } else {
                let re = regex::Regex::new(r"\d+")?;
                let nums = re.find_iter(line.as_str()).map(|s| s.as_str().parse::<isize>().unwrap()).collect::<Vec<_>>();
                inputs.push(Position::new([nums[0], nums[1]]));
            }
        }
        Ok(())
    })
}

