use std::{collections::{HashMap, HashSet}, fmt::Display};

use advent2024::{advent_main, all_lines, Part};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        if options.len() > 0 {
            let stones = Stones::new(filename)?;
            visualize(stones, options[0]);
        } else {
            match part {
                Part::One => {
                    let mut stones = Stones::new(filename)?;
                    for _ in 0..25 {
                        stones.blink();
                    }
                    println!("{}", stones.len());
                }
                Part::Two => {
                    let mut table = StoneTable::new(filename)?;

                    println!("{}", table.count());
                }
            }
        }
        Ok(())
    })
}

struct StoneTable {
    blinks2stones: Vec<HashMap<Stone, u128>>
}

impl StoneTable {
    fn new(filename: &str) -> anyhow::Result<Self> {
        let line = all_lines(filename)?.next().unwrap();
        Ok(Self {
            blinks2stones: vec![line.split_whitespace().map(|sn| (Stone::new(sn.parse::<u128>().unwrap()), 1)).collect()]
        })
    }

    fn count(&self) -> u128 {
        self.blinks2stones.last().unwrap().values().sum()
    }

    fn blink(&mut self) {
        let mut new_line = HashMap::new();
        for (stone, count) in self.blinks2stones.last().unwrap().iter() {
            for new_stone in stone.blink() {
                
            }
        }
        self.blinks2stones.push(new_line);
    }
}

struct Stones {
    stones: Vec<Stone>,
}

impl Stones {
    fn new(filename: &str) -> anyhow::Result<Self> {
        let line = all_lines(filename)?.next().unwrap();
        Ok(Self {
            stones: line.split_whitespace()
                .map(|sn| Stone::new(sn.parse::<u128>().unwrap()))
                .collect(),
        })
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

    fn blink_unseen_only(&mut self, seen: &HashSet<Stone>) {
        let mut blinked = vec![];
        for stone in self.stones.iter() {
            for s in stone.blink() {
                if !seen.contains(&s) {
                    blinked.push(s);
                }
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
            write!(f, "{stone} ")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Stone {
    number: u128,
}

impl Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl Stone {
    fn new(number: u128) -> Self {
        Self { number }
    }

    fn as_string(&self) -> String {
        format!("{}", self)
    }

    fn blink(&self) -> Vec<Self> {
        if self.number == 0 {
            vec![Self::new(1)]
        } else {
            let s = self.as_string();
            if s.len() % 2 == 0 {
                let halfway = s.len() / 2;
                vec![&s[..halfway], &s[halfway..]]
                    .iter()
                    .map(|sub| Self::new(sub.parse::<u128>().unwrap()))
                    .collect()
            } else {
                vec![Self::new(self.number * 2024)]
            }
        }
    }
}

fn visualize(mut stones: Stones, opt: &str) {
    let n = opt.find(':').unwrap();
    let n = opt[(n + 1)..].parse::<usize>().unwrap();
    if opt.starts_with("-explore") {
        for i in 0..n {
            stones.blink();
            println!("Step {}: {}", (i + 1), stones);
        }
    } else if opt.starts_with("-count") {
        for i in 0..n {
            stones.blink();
            println!("Step {}: {}", (i + 1), stones.len());
        }
    } else if opt.starts_with("-valueset") {
        let mut stones = Stones {
            stones: vec![Stone { number: n as u128 }],
        };
        let mut seen = HashSet::new();
        let mut i = 0;
        loop {
            stones.blink_unseen_only(&seen);
            let unseen = stones.stones.iter().filter(|s| !seen.contains(*s)).count();
            println!("Step {}: (unseen: {}) {}", (i + 1), unseen, stones.len());
            if unseen == 0 {
                break;
            }
            for stone in stones.stones.iter() {
                seen.insert(*stone);
            }
            i += 1;
        }
    } else if opt.starts_with("-value") {
        let mut stones = Stones {
            stones: vec![Stone { number: n as u128 }],
        };
        for i in 0..10 {
            stones.blink();
            println!("Step {} ({}): {}", (i + 1), stones.len(), stones);
        }
    } else if opt.starts_with("-sets") {
        let mut seen = HashSet::new();
        for i in 0..n {
            stones.blink_unseen_only(&seen);
            let unseen = stones.stones.iter().filter(|s| !seen.contains(*s)).count();
            println!("Step {}: (unseen: {}) {}", (i + 1), unseen, stones.len());
            for stone in stones.stones.iter() {
                seen.insert(*stone);
            }
        }
    }
}