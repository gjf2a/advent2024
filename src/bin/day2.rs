use advent2024::{all_lines, advent_main, Part};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let result = all_lines(filename)?
            .filter(|line| match part {
                Part::One => safe_line(&line2nums(line.as_str())),
                Part::Two => safe_line_2(line.as_str()),
            })
            .count();
        println!("{result}");
        Ok(())
    })
}

#[derive(Eq, PartialEq, Copy, Clone)]
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
        Some(dir) => (1..nums.len()).all(|i| safe_pair(nums[i - 1], nums[i], dir)),
    }
}

fn safe_pair(n1: i64, n2: i64, dir: Dir) -> bool {
    return (n1 - n2).abs() <= 3 && Dir::new(n1, n2).map_or(false, |d| d == dir);
}

fn without_element(nums: &Vec<i64>, target: usize) -> Vec<i64> {
    nums.iter()
        .enumerate()
        .filter(|(i, _)| *i != target)
        .map(|(_, v)| *v)
        .collect()
}

fn safe_line_2(line: &str) -> bool {
    let nums = line2nums(line);
    (0..nums.len()).any(|i| safe_line(&without_element(&nums, i)))
}
