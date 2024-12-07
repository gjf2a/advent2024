use advent2024::{all_lines, chooser_main, ComboIterator, Part};
use enum_iterator::{all, Sequence};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        println!("{filename} {part:?}");
        let mut total = 0;
        for line in all_lines(filename)? {
            let (target, nums) = parse(line);
            let value = match part {
                Part::One => matching_op_combo([Op::Plus, Op::Times].iter().copied(), target, &nums),
                Part::Two => matching_op_combo(all::<Op>(), target, &nums)
            };
            if value.is_some() {
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
    (
        target,
        parts_a
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect(),
    )
}

fn matching_op_combo(iter: impl Iterator<Item = Op> + Clone, target: i64, nums: &Vec<i64>) -> Option<Vec<Op>> {
    ComboIterator::new(iter, nums.len() - 1).find(|combo| Op::apply(combo, nums) == target)
}

#[derive(Copy, Clone, Sequence, Debug)]
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
            Self::Concat => format!("{op1}{op2}").parse().unwrap(),
        }
    }
}
