use std::{fmt::Display, ops::Index};

use advent2024::{advent_main, all_lines};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let mut stones = Stones::new(filename)?;
        if options.len() > 0 {
            if options[0].starts_with("-explore") {
                let n = options[0].find(':').unwrap();
                let n = options[0][(n + 1)..].parse::<usize>().unwrap();
                for i in 0..n {
                    stones.blink();
                    println!("Step {}: {}", (i + 1), stones);
                }
            } else if options[0].starts_with("-count") {
                let n = options[0].find(':').unwrap();
                let n = options[0][(n + 1)..].parse::<usize>().unwrap();
                for i in 0..n {
                    stones.blink();
                    println!("Step {}: {}", (i + 1), stones.len());
                }
            }
        }
        Ok(())
    })
}

struct Stones {
    stones: Vec<Stone>
}

impl Stones {
    fn new(filename: &str) -> anyhow::Result<Self> {
        Ok(Self {stones: all_lines(filename)?.next().unwrap().split_whitespace().map(|sn| Stone {number: sn.parse::<u128>().unwrap()}).collect()})
    }

    fn blink(&mut self) {
        let mut blinked = vec![];
        for stone in self.stones.iter() {
            for s in stone.blink() {
                blinked.push(s);
            }
        }
        self.stones = blinked;
    }

    fn len(&self) -> usize {
        self.stones.len()
    }
}

impl Display for Stones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stone in self.stones.iter() {
            write!(f, "{} ", stone.as_string())?;
        }
        Ok(())
    }
}

struct Stone {
    number: u128
}

impl Stone {
    fn as_string(&self) -> String {
        format!("{}", self.number)
    }

    fn blink(&self) -> Vec<Self> {
        if self.number == 0 {
            vec![Self {number: 1}]
        } else {
            let s = self.as_string();
            if s.len() % 2 == 0 {
                let halfway = s.len() / 2;
                vec![&s[..halfway], &s[halfway..]].iter().map(|sub| Self {number: sub.parse::<u128>().unwrap()}).collect()
            } else {
                vec![Self {number: self.number * 2024}]
            }
        }
    }
}