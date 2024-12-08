use advent2024::{advent_main, all_lines, combinations::ComboIterator, Part};

const PART_1: [Op; 2] = [Op::Plus, Op::Times];
const PART_2: [Op; 3] = [Op::Plus, Op::Times, Op::Concat];

// NOTE: Recursive solution is my translation of Mark Goadrich's Go solution:
// https://github.com/mgoadric/AdventOfCode/blob/main/2024/Go/day7/day7.go

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let early = options.contains(&"-early");
        let recursive = early || options.contains(&"-recursive");
        let ops = match part {
            Part::One => &PART_1[..],
            Part::Two => &PART_2[..],
        };
        let mut total = 0;
        for line in all_lines(filename)? {
            let (target, nums) = parse(line);
            if recursive && solve_recursive(early, ops, target, 0, &nums[..])
                || !recursive && solve_iterator(ops.iter().copied(), target, &nums).is_some()
            {
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

fn solve_recursive(early: bool, ops: &[Op], target: i64, current: i64, nums: &[i64]) -> bool {
    if early && current > target {
        false
    } else if nums.len() == 0 {
        target == current
    } else {
        ops.iter()
            .any(|op| solve_recursive(early, ops, target, op.op(current, nums[0]), &nums[1..]))
    }
}

fn solve_iterator(
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
            Self::Concat => op1 * 10_i64.pow(log10(op2) + 1) + op2,
        }
    }
}

fn log10(mut n: i64) -> u32 {
    let mut result = 0;
    while n >= 10 {
        n /= 10;
        result += 1;
    }
    result
}
