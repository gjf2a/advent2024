use advent_code_lib::{all_lines, chooser_main, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        match part {
            Part::One => {
                let result = all_lines(filename)?
                    .filter(|line| safe_line(line.as_str()))
                    .count();
                println!("{result}");
            }
            Part::Two => {}
        }

        Ok(())
    })
}

#[derive(Eq, PartialEq)]
enum Dir {
    Up,
    Down,
}

impl Dir {
    fn new(n1: i64, n2: i64) -> Option<Self> {
        if n1 < n2 {
            Some(Self::Up)
        } else if n1 > n2 {
            Some(Self::Down)
        } else {
            None
        }
    }
}

fn safe_line(line: &str) -> bool {
    let mut nums = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    let mut prev = nums.next().unwrap();
    let mut current = nums.next().unwrap();
    match Dir::new(prev, current) {
        None => false,
        Some(dir) => loop {
            if (prev - current).abs() > 3 || Dir::new(prev, current).map_or(true, |d| d != dir) {
                return false;
            }
            match nums.next() {
                None => return true,
                Some(n) => {
                    prev = current;
                    current = n;
                }
            }
        },
    }
}
