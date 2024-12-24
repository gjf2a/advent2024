use std::{collections::BTreeMap, str::FromStr};

use advent2024::{advent_main, all_lines};
use anyhow::anyhow;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        println!("{filename} {part:?}");
        let mut circuit = Circuit::from_file(filename)?;
        println!("{}", circuit.part1());
        Ok(())
    })
}

struct Circuit {
    values: BTreeMap<String, u128>,
    pending: Vec<Gate>,
}

impl Circuit {
    fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut lines = all_lines(filename)?;
        let mut values = BTreeMap::new();
        for init in lines.by_ref().take_while(|line| line.len() > 0) {
            let (name, value) = init.split(": ").collect_tuple().unwrap();
            values.insert(name.to_string(), value.parse::<u128>().unwrap());
        }

        let pending = lines.map(|line| line.parse::<Gate>().unwrap()).collect();
        Ok(Self { values, pending })
    }

    fn extract_num_with(&self, prefix: &str) -> u128 {
        self.values
            .iter()
            .rev()
            .filter(|(n, _)| n.starts_with(prefix))
            .map(|(_, v)| *v)
            .reduce(|a, b| a * 2 + b)
            .unwrap()
    }

    fn part1(&mut self) -> u128 {
        while !self.pending.is_empty() {
            let mut new_pending = self
                .pending
                .iter()
                .filter_map(|gate| gate.apply(&mut self.values))
                .collect();
            std::mem::swap(&mut self.pending, &mut new_pending);
        }

        self.extract_num_with("z")
    }
}

#[derive(Clone, Debug)]
struct Args {
    a: String,
    b: String,
    c: String,
}

#[derive(Clone, Debug)]
enum Gate {
    And(Args),
    Or(Args),
    Xor(Args),
}

impl Gate {
    fn ops(&self, values: &BTreeMap<String, u128>) -> Option<(u128, u128, String)> {
        let args = match self {
            Gate::And(args) => args,
            Gate::Or(args) => args,
            Gate::Xor(args) => args,
        };
        if let Some(a) = values.get(args.a.as_str()) {
            if let Some(b) = values.get(args.b.as_str()) {
                return Some((*a, *b, args.c.clone()));
            }
        }
        None
    }

    fn eval(&self, a: u128, b: u128) -> u128 {
        match self {
            Gate::And(_) => a & b,
            Gate::Or(_) => a | b,
            Gate::Xor(_) => a ^ b,
        }
    }

    fn apply(&self, values: &mut BTreeMap<String, u128>) -> Option<Self> {
        self.ops(values)
            .map(|(a, b, c)| {
                values.insert(c, self.eval(a, b));
            })
            .map_or(Some(self.clone()), |_| None)
    }
}

impl FromStr for Gate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, op, b, _, c) = s.split_whitespace().collect_tuple().unwrap();
        let args = Args {
            a: a.to_string(),
            b: b.to_string(),
            c: c.to_string(),
        };
        match op {
            "AND" => Ok(Self::And(args)),
            "OR" => Ok(Self::Or(args)),
            "XOR" => Ok(Self::Xor(args)),
            _ => Err(anyhow!("No match")),
        }
    }
}
