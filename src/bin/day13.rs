use advent2024::{
    advent_main, all_lines, multidim::Position, Part,
};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
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
                total += cheapest(goal, a, b).unwrap_or(0);
                inputs = vec![];
            }
        }
        println!("{total}");
        Ok(())
    })
}

fn cheapest(goal: Position, a: Position, b: Position) -> Option<isize> {
    // Used the substitution method to solve a system of linear equations.
    let push_a = (b[1] * goal[0] - b[0] * goal[1]) / (a[0] * b[1] - b[0] * a[1]);
    let push_b = (goal[1] - a[1] * push_a) / b[1];
    if (0..2).all(|i| a[i] * push_a + b[i] * push_b == goal[i]) {
        Some(push_a * 3 + push_b)
    } else {
        None
    }
}
