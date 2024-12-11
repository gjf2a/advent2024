use advent2024::{advent_main, all_lines, Part};
use hash_histogram::HashHistogram;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let iterations = match part {
            Part::One => 25,
            Part::Two => 75,
        };
        let mut table = StoneTable::new(filename)?;
        for _ in 0..iterations {
            table.blink();
        }
        println!("{}", table.count());
        Ok(())
    })
}

struct StoneTable {
    blinks2stones: Vec<HashHistogram<Stone, u128>>,
}

impl StoneTable {
    fn new(filename: &str) -> anyhow::Result<Self> {
        let line = all_lines(filename)?.next().unwrap();
        Ok(Self {
            blinks2stones: vec![line
                .split_whitespace()
                .map(|sn| Stone::new(sn.parse::<u128>().unwrap()))
                .collect()],
        })
    }

    fn count(&self) -> u128 {
        self.blinks2stones.last().unwrap().total_count()
    }

    fn blink(&mut self) {
        let mut new_line = HashHistogram::new();
        for (stone, count) in self.blinks2stones.last().unwrap().iter() {
            for new_stone in stone.blink() {
                new_line.bump_by(&new_stone, *count);
            }
        }
        self.blinks2stones.push(new_line);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Stone {
    number: u128,
}

impl Stone {
    fn new(number: u128) -> Self {
        Self { number }
    }

    fn as_string(&self) -> String {
        format!("{}", self.number)
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
