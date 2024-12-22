use std::collections::HashMap;

use advent2024::{advent_main, all_lines, Part};
use hash_histogram::HashHistogram;
use num::Integer;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        if options.contains(&"-period") {
            periods(filename)?;
        }
        match part {
            Part::One => part1(filename)?,
            Part::Two => part2(filename)?,
        }
        Ok(())
    })
}

fn mix_and_prune(a: i128, b: i128) -> i128 {
    (a ^ b) & 0xffffff
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

fn part1(filename: &str) -> anyhow::Result<()> {
    let total = all_lines(filename)?
        .map(|line| line.parse::<i128>().unwrap())
        .map(|n| SecretNumberSequence::new(n).skip(2000).next().unwrap())
        .sum::<i128>();
    println!("{total}");
    Ok(())
}

fn part2(filename: &str) -> anyhow::Result<()> {
    let mut options = HashHistogram::new();
    for line in all_lines(filename)? {
        let line = line.parse::<i128>().unwrap();
        update_option_totals(line, &mut options);
    }
    println!("{:?}", options.ranking_with_counts()[0]);
    Ok(())
}

fn update_option_totals(line: i128, options: &mut HashHistogram<Changes>) {
    let mut changes = Changes::default();
    let mut sequence = SecretNumberSequence::new(line).take(2000);
    let mut prev = sequence.by_ref().next().unwrap().mod_floor(&10);
    let mut change_map = HashMap::new();
    for num in sequence {
        let digit = num.mod_floor(&10);
        changes.add(digit - prev);
        if changes.full() {
            if !change_map.contains_key(&changes) {
                change_map.insert(changes, digit as usize);
            }
        }
        prev = digit;
    }
    for (option, value) in change_map.iter() {
        options.bump_by(option, *value);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Changes {
    changes: [i128; 4],
    in_use: usize,
}

impl Changes {
    fn full(&self) -> bool {
        self.in_use == self.changes.len()
    }

    fn add(&mut self, change: i128) {
        if self.full() {
            for i in 0..(self.in_use - 1) {
                self.changes[i] = self.changes[i + 1];
            }
            self.changes[self.in_use - 1] = change;
        } else {
            self.changes[self.in_use] = change;
            self.in_use += 1;
        }
    }
}

fn periods(filename: &str) -> anyhow::Result<()> {
    for line in all_lines(filename)? {
        let n = line.parse::<i128>().unwrap();
        match period(n) {
            Some((period_start, period)) => {
                println!("{n}: from {period_start}, {period}");
            }
            None => {
                println!("{n}: No period found in 2000 values.")
            }
        }
    }
    Ok(())
}

fn period(start: i128) -> Option<(usize, usize)> {
    let mut starts = HashMap::new();
    for (i, n) in SecretNumberSequence::new(start).enumerate().take(2000) {
        match starts.get(&n) {
            None => {
                starts.insert(n, i);
            }
            Some(prev) => {
                return Some((*prev, i - *prev));
            }
        }
    }
    None
}
