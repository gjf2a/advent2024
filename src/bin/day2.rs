use advent_code_lib::{all_lines, chooser_main, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        match part {
            Part::One => {
                let result = all_lines(filename)?
                    .filter(|line| safe_line(&line2nums(line.as_str())))
                    .count();
                println!("{result}");
            }
            Part::Two => {
                let result = all_lines(filename)?
                    .filter(|line| safe_line_2(line.as_str()))
                    .count();
                println!("{result}");
            }
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

fn line2nums(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn safe_line(nums: &Vec<i64>) -> bool {
    match Dir::new(nums[0], nums[1]) {
        None => false,
        Some(dir) => {
            for i in 1..nums.len() {
                if (nums[i] - nums[i - 1]).abs() > 3
                    || Dir::new(nums[i - 1], nums[i]).map_or(true, |d| d != dir)
                {
                    return false;
                }
            }
            true
        }
    }
}

fn without_element(nums: &Vec<i64>, target: usize) -> Vec<i64> {
    nums.iter()
        .enumerate()
        .filter(|(i, _)| *i != target)
        .map(|(_, v)| *v)
        .collect()
}

fn safe_line_2(line: &str) -> bool {
    let nums = line
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    for i in 0..nums.len() {
        if safe_line(&without_element(&nums, i)) {
            return true;
        }
    }
    false
}
