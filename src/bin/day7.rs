use advent2024::{all_lines, chooser_main, combinations::ComboIterator, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        println!("{filename} {part:?}");
        let mut total = 0;
        for line in all_lines(filename)? {
            let (target, nums) = parse(line);
            let iter = match part {
                Part::One => [Op::Plus, Op::Times].iter(),
                Part::Two => [Op::Plus, Op::Times, Op::Concat].iter(),
            };
            if matching_op_combo(iter.copied(), target, &nums).is_some() {
                total += target;
            }
        }
        println!("{total}");
        Ok(())
    })
}

fn parse(line: String) -> (i64, Vec<i64>) {
    let mut parts_a = line.split(':');
    let target = parts_a.next().unwrap().parse::<i64>().unwrap();
    let nums = parts_a
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    (target, nums)
}

fn matching_op_combo(
    iter: impl Iterator<Item = Op> + Clone,
    target: i64,
    nums: &Vec<i64>,
) -> Option<Vec<Op>> {
    ComboIterator::new(iter, nums.len() - 1).find(|combo| Op::apply(combo, nums) == target)
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Plus,
    Times,
    Concat,
}

impl Op {
    fn apply(ops: &Vec<Self>, nums: &Vec<i64>) -> i64 {
        assert_eq!(ops.len() + 1, nums.len());
        let mut total = ops[0].op(nums[0], nums[1]);
        for i in 1..ops.len() {
            total = ops[i].op(total, nums[i + 1]);
        }
        total
    }

    fn op(&self, op1: i64, op2: i64) -> i64 {
        match self {
            Self::Plus => op1 + op2,
            Self::Times => op1 * op2,
            Self::Concat => concat(op1, op2),
        }
    }
}

fn concat(n1: i64, n2: i64) -> i64 {
    n1 * 10_i64.pow(log10(n2) + 1) + n2
}

fn log10(mut n: i64) -> u32 {
    let mut result = 0;
    while n >= 10 {
        n /= 10;
        result += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{concat, log10};

    #[test]
    fn test_log() {
        for (n, l) in [
            (1, 0),
            (9, 0),
            (10, 1),
            (99, 1),
            (100, 2),
            (999, 2),
            (1000, 3),
        ] {
            assert_eq!(log10(n), l);
        }
    }

    #[test]
    fn test_concat() {
        for (n1, n2) in [(12, 345), (10, 20), (133, 8)] {
            assert_eq!(concat(n1, n2), format!("{n1}{n2}").parse::<i64>().unwrap());
        }
    }
}