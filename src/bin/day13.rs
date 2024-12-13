use advent2024::{advent_main, all_lines, multidim::Position};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let mut inputs = vec![];
        let mut total = 0;
        for line in all_lines(filename)? {
            if inputs.len() == 3 {
                total += brute_force_tokens(inputs, 100).unwrap_or(0);
                inputs = vec![];
            } else {
                let re = regex::Regex::new(r"\d+")?;
                let nums = re.find_iter(line.as_str()).map(|s| s.as_str().parse::<isize>().unwrap()).collect::<Vec<_>>();
                inputs.push(Position::new([nums[0], nums[1]]));
            }
        }
        println!("{total}");
        Ok(())
    })
}

fn brute_force_tokens(inputs: Vec<Position>, max_presses: isize) -> Option<isize> {
    let button_a = inputs[0];
    let button_b = inputs[1];
    let goal = inputs[2];
    let mut best = None;
    for a in 0..max_presses {
        for b in 0..max_presses {
            let location = button_a * a + button_b * b;
            if location == goal {
                let tokens = a * 3 + b;
                if best.map_or(true, |b| tokens < b) {
                    best = Some(tokens);
                }
            }
        }
    }
    best
}