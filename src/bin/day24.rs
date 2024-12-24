use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use advent2024::{advent_main, all_lines, search_iter::BfsIter, Part};
use anyhow::anyhow;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let circuit = Circuit::from_file(filename)?;
        if options.contains(&"-showzs") {
            show_bad_zs(circuit);
        } else if options.contains(&"-singles") {
            show_single_ancestors(circuit);
        } else {
            match part {
                Part::One => part1(circuit),
                Part::Two => part2(circuit),
            }
        }
        Ok(())
    })
}

fn part1(mut circuit: Circuit) {
    circuit.run();
    println!("{}", circuit.extract_num_with("z"));
}

fn part2(circuit: Circuit) {
    let bad_zs = circuit.bad_zs();
}

#[derive(Clone)]
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

    fn run(&mut self) {
        while !self.pending.is_empty() {
            let mut new_pending = self
                .pending
                .iter()
                .filter_map(|gate| gate.apply(&mut self.values))
                .collect();
            std::mem::swap(&mut self.pending, &mut new_pending);
        }
    }

    fn gate_for(&self, var: &str) -> Gate {
        self.pending
            .iter()
            .find(|g| g.output() == var)
            .unwrap()
            .clone()
    }

    fn swapped_outputs_for(&self, g1: &Gate, g2: &Gate) -> Self {
        Self {
            values: self.values.clone(),
            pending: self
                .pending
                .iter()
                .map(|g| {
                    if g == g1 {
                        g.with_new_output(g2.output())
                    } else if g == g2 {
                        g.with_new_output(g1.output())
                    } else {
                        g.clone()
                    }
                })
                .collect(),
        }
    }

    fn ancestors_of(&self, var: &str) -> BTreeSet<Gate> {
        BfsIter::new(self.gate_for(var), |v| {
            let mut ancestors = vec![];
            for gate in self.pending.iter() {
                if v.has_input(gate.output()) {
                    ancestors.push(gate.clone());
                }
            }
            ancestors
        })
        .collect()
    }

    fn bad_zs(&self) -> BTreeSet<String> {
        let mut test = self.clone();
        let x = test.extract_num_with("x");
        let y = test.extract_num_with("y");
        test.run();
        let z = test.extract_num_with("z");
        let goal = x + y;
        let wrong = z ^ goal;
        (0..)
            .take_while(|i| wrong > 2_u128.pow(*i))
            .filter(|i| (wrong >> *i) & 1 == 1)
            .map(|i| {
                let i = if i >= 10 {
                    format!("{i}")
                } else {
                    format!("0{i}")
                };
                format!("z{i}")
            })
            .collect()
    }

    fn single_ancestor_bad(&self) -> Gate {
        self.bad_zs()
            .iter()
            .map(|z| self.ancestors_of(z.as_str()))
            .find(|a| a.len() == 1)
            .map(|a| a.iter().next().unwrap().clone())
            .unwrap()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Args {
    a: String,
    b: String,
    c: String,
}

impl Args {
    fn with_new_output(&self, new_c: &str) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            c: new_c.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Gate {
    And(Args),
    Or(Args),
    Xor(Args),
}

impl Gate {
    fn ops(&self, values: &BTreeMap<String, u128>) -> Option<(u128, u128, String)> {
        if let Some(a) = values.get(self.args().a.as_str()) {
            if let Some(b) = values.get(self.args().b.as_str()) {
                return Some((*a, *b, self.args().c.clone()));
            }
        }
        None
    }

    fn has_input(&self, candidate: &str) -> bool {
        self.args().a.as_str() == candidate || self.args().b.as_str() == candidate
    }

    fn with_new_output(&self, output: &str) -> Self {
        let new_args = self.args().with_new_output(output);
        match self {
            Self::And(_) => Self::And(new_args),
            Self::Or(_) => Self::Or(new_args),
            Self::Xor(_) => Self::Xor(new_args),
        }
    }

    fn args(&self) -> &Args {
        match self {
            Gate::And(args) => args,
            Gate::Or(args) => args,
            Gate::Xor(args) => args,
        }
    }

    fn output(&self) -> &str {
        self.args().c.as_str()
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

fn show_bad_zs(circuit: Circuit) {
    let mut test = circuit.clone();
    let x = test.extract_num_with("x");
    let y = test.extract_num_with("y");
    test.run();
    let z = test.extract_num_with("z");
    let goal = x + y;
    let wrong = z ^ goal;
    println!("{wrong:#b}");
    println!("{:?}", circuit.bad_zs());
    let union = circuit
        .bad_zs()
        .iter()
        .map(|z| circuit.ancestors_of(z.as_str()))
        .reduce(|a, b| a.union(&b).cloned().collect::<BTreeSet<_>>())
        .unwrap();
    println!(
        "union size: {} ({} gates total)",
        union.len(),
        circuit.pending.len()
    );

    ancestor_analysis(&circuit);
    let for_sure = circuit.single_ancestor_bad();
    for gate in circuit.bad_zs().iter().filter(|v| for_sure.args().c != **v) {
        let alternative = circuit.swapped_outputs_for(&for_sure, &circuit.gate_for(gate.as_str()));
        println!("vs {gate}:");
        ancestor_analysis(&alternative);
    }
}

fn ancestor_analysis(circuit: &Circuit) {
    for z in circuit.bad_zs().iter() {
        let ancestors = circuit.ancestors_of(z.as_str());
        if circuit.ancestors_of(z.as_str()).len() == 1 {
            println!("z: {z} ancestors: {ancestors:?}");
        } else {
            println!("z: {z} # ancestors: {}", ancestors.len());
        }
        println!();
    }
}

fn show_single_ancestors(mut circuit: Circuit) {
    let mut all_singles = BTreeSet::new();
    for bad_z in circuit.bad_zs() {
        println!("bad z: {bad_z}");
        let ancestors = circuit.ancestors_of(bad_z.as_str());
        for ancestor in ancestors {
            if circuit.ancestors_of(ancestor.output()).len() == 1 {
                println!("single ancestor: {ancestor:?}");
                all_singles.insert(ancestor);
            }
        }
        println!();
    }
    println!("Total singles: {}", all_singles.len());
    let all_outputs = circuit.pending.iter().map(|g| g.output().to_string()).collect_vec();

    circuit.run();
    let ones = all_singles.iter().filter(|s| *circuit.values.get(s.output()).unwrap() == 1).count();
    println!("Singles: Ones: {ones} Zeros: {}", all_singles.len() - ones);

    let ones = all_outputs.iter().filter(|o| *circuit.values.get(o.as_str()).unwrap() == 1).count();
    println!("All: Ones: {ones} Zeros: {}", all_outputs.len() - ones);
}
