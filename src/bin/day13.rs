use std::cmp::min;

use advent2024::{advent_main, all_lines, multidim::Position, Part};
use num::integer::gcd;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let mut inputs = vec![];
        let mut total = 0;
        for line in all_lines(filename)?.filter(|line| line.len() > 0) {
            let re = regex::Regex::new(r"\d+")?;
            let nums = re.find_iter(line.as_str()).map(|s| s.as_str().parse::<isize>().unwrap()).collect::<Vec<_>>();
            inputs.push(Position::new([nums[0], nums[1]]));
            if inputs.len() == 3 {
                let a = inputs[0];
                let b = inputs[1];
                let goal = match part {
                    Part::One => inputs[2],
                    Part::Two => inputs[2] + Position::new([10000000000000, 10000000000000]),
                };
                let cost = if options.contains(&"-brute") {
                    brute_force_tokens(inputs, 100)
                } else {
                    cheapest(goal, a, b)
                };
                println!("{cost:?}");
                total += cost.unwrap_or(0);
                inputs = vec![];
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
                if best.map_or(true, |(b, _, _)| tokens < b) {
                    best = Some((tokens, a, b));
                }
            }
        }
    }
    println!("Winner: {best:?}");
    best.map(|b| b.0)
}

fn cheapest(goal: Position, a: Position, b: Position) -> Option<isize> {
    let exists = a.values().zip(b.values()).zip(goal.values()).all(|((ca, cb), cg)| cg % gcd(ca, cb) == 0);
    if exists {
        todo!()
    } else {
        None
    }
}