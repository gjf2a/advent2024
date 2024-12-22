use advent2024::{advent_main, all_lines};
use num::Integer;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let total = all_lines(filename)?
            .map(|line| line.parse::<u128>().unwrap())
            .map(|n| SecretNumberSequence::new(n).skip(2000).next().unwrap())
            .sum::<u128>();
        println!("{total}");
        Ok(())
    })
}

fn mix_and_prune(a: u128, b: u128) -> u128 {
    (a ^ b).mod_floor(&16777216)
}

struct SecretNumberSequence {
    secret: u128,
}

impl SecretNumberSequence {
    fn new(start: u128) -> Self {
        Self { secret: start }
    }

    fn mix_and_prune<S: Fn(u128) -> u128>(&mut self, f: S) {
        self.secret = mix_and_prune(self.secret, f(self.secret));
    }
}

impl Iterator for SecretNumberSequence {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.secret;
        self.mix_and_prune(|s| s * 64);
        self.mix_and_prune(|s| s / 32);
        self.mix_and_prune(|s| s * 2048);
        Some(result)
    }
}
