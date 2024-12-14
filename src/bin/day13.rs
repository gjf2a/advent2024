use advent2024::{
    advent_main, all_lines, extended_euclid::LinearDiophantinePositive, multidim::Position, Part,
};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let mut inputs = vec![];
        let mut total = 0;
        for line in all_lines(filename)?.filter(|line| line.len() > 0) {
            let re = regex::Regex::new(r"\d+")?;
            let nums = re
                .find_iter(line.as_str())
                .map(|s| s.as_str().parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            inputs.push(Position::new([nums[0], nums[1]]));
            if inputs.len() == 3 {
                let a = inputs[0];
                let b = inputs[1];
                let goal = match part {
                    Part::One => inputs[2],
                    Part::Two => inputs[2] + Position::from((10000000000000, 10000000000000)),
                };
                println!("{a} {b} {goal}");
                let cost = if options.contains(&"-brute") {
                    brute_force_tokens(inputs, 100)
                } else {
                    cheapest(goal, a, b)
                };
                println!("cost: {cost:?}");
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
    // a[0] * x + b[0] * y = goal[0]
    // a[1] * x + b[1] * y = goal[1]
    //
    // y = (goal[1] - a[1] * x) / b[1]
    // a[0] * x + b[0] * ((goal[1] - a[1] * x) / b[1]) = goal[0]
    // a[0] * b[1] * x + b[0] * goal[1] - b[0] * a[1] * x = b[1] * goal[0]
    // a[0] * b[1] * x - b[0] * a[1] * x = b[1] * goal[0] - b[0] * goal[1]
    // x = (b[1] * goal[0] - b[0] * goal[1]) / (a[0] * b[1] - b[0] * a[1])

    let push_a = (b[1] * goal[0] - b[0] * goal[1]) / (a[0] * b[1] - b[0] * a[1]);
    let push_b = (goal[1] - a[1] * push_a) / b[1];
    if (0..2).all(|i| a[i] * push_a + b[i] * push_b == goal[i]) {
        Some(push_a * 3 + push_b)
    } else {
        None
    }
}
