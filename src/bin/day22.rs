use advent2024::{advent_main, all_lines, Part};
use num::Integer;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        match part {
            Part::One => {
                let total = all_lines(filename)?
                    .map(|line| line.parse::<i128>().unwrap())
                    .map(|n| SecretNumberSequence::new(n).skip(2000).next().unwrap())
                    .sum::<i128>();
                println!("{total}");
            }
            Part::Two => {
                todo!();
            }
        }
        Ok(())
    })
}

fn mix_and_prune(a: i128, b: i128) -> i128 {
    (a ^ b).mod_floor(&16777216)
}

struct SecretNumberSequence {
    secret: i128,
}

impl SecretNumberSequence {
    fn new(start: i128) -> Self {
        Self { secret: start }
    }

    fn mix_and_prune<S: Fn(i128) -> i128>(&mut self, f: S) {
        self.secret = mix_and_prune(self.secret, f(self.secret));
    }
}

impl Iterator for SecretNumberSequence {
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.secret;
        self.mix_and_prune(|s| s * 64);
        self.mix_and_prune(|s| s / 32);
        self.mix_and_prune(|s| s * 2048);
        Some(result)
    }
}
