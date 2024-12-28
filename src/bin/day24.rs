use std::{
    collections::{BTreeMap, BTreeSet, HashMap}, fmt::Display, str::FromStr, sync::Arc
};

use advent2024::{
    advent_main, all_lines,
    graph::{graphviz_directed, AdjacencySets},
    search_iter::BfsIter,
    Part,
};
use anyhow::anyhow;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let circuit = Circuit::from_file(filename)?;
        if options.contains(&"-showzs") {
            show_bad_zs(circuit);
        } else if options.contains(&"-singles") {
            show_single_ancestors(circuit);
        } else if options.contains(&"-dot") {
            let (graph, labels) = circuit.directed_edges();
            graphviz_directed(graph.iter().cloned(), "day24.dot", &labels)?;
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
    circuit.run_to_completion();
    println!("{}", circuit.extract_num_with("z"));
}

fn part2(circuit: Circuit) {
    let mut graph = AdjacencySets::default();
    let (edges, _) = circuit.directed_edges();
    for (src, dest) in edges {
        graph.connect(src.as_str(), dest.as_str());
    }
    let graph = graph;
    let in_degrees = graph.in_degrees();
    let topo = graph.topologial_ordering().unwrap();
    let topo_output = topo
        .iter()
        .filter(|s| *(in_degrees.get(*s).unwrap()) == 2)
        .cloned()
        .collect_vec();
    let pairs = BTreeMap::new();
    if !search(&circuit, &topo_output, 0, &pairs) {
        println!("Failed");
    }
}

fn search(circuit: &Circuit, topo_output: &Vec<String>, start: usize, pairs: &BTreeMap<String,String>) -> bool {
    if pairs.len() == 4 {
        let test = circuit.swapped_output_pairs(&pairs);
        if let Some(bad_zs) = test.bad_zs() {
            if bad_zs.len() == 0 {
                let mut result = BTreeSet::new();
                for (k, v) in pairs.iter() {
                    result.insert(k);
                    result.insert(v);
                }
                let output = result.iter().join(",");
                println!("{output}");
                return true;
            }
        }
        false
    } else {
        for i in start..topo_output.len() {
            for j in (i + 1)..topo_output.len() {
                if start == 0 {
                    println!("From ({}, {})", i, j);
                }
                let mut pairs = pairs.clone();
                pairs.insert(topo_output[i].to_string(), topo_output[j].to_string());
                if search(circuit, topo_output, j + 1, &pairs) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Circuit {
    values: BTreeMap<String, u128>,
    pending: BTreeMap<String, Gate>,
}

impl Circuit {
    fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut lines = all_lines(filename)?;
        let mut values = BTreeMap::new();
        for init in lines.by_ref().take_while(|line| line.len() > 0) {
            let (name, value) = init.split(": ").collect_tuple().unwrap();
            values.insert(name.to_string(), value.parse::<u128>().unwrap());
        }

        let pending = lines
            .map(|line| line.parse::<Gate>().unwrap())
            .map(|g| (g.output().to_string(), g))
            .collect();
        Ok(Self { values, pending })
    }

    fn directed_edges(&self) -> (Vec<(String, String)>, HashMap<(String,String), String>) {
        let mut labels = HashMap::new();
        let mut graph = vec![];
        BfsIter::multi_start(
            self.pending.keys().cloned().filter(|k| k.starts_with("z")),
            |output| match self.pending.get(output) {
                None => vec![],
                Some(gate) => {
                    labels.insert((gate.args().a.clone(), output.clone()), gate.type_name());
                    labels.insert((gate.args().b.clone(), output.clone()), gate.type_name());
                    graph.push((gate.args().a.clone(), output.clone()));
                    graph.push((gate.args().b.clone(), output.clone()));
                    vec![gate.args().a.clone(), gate.args().b.clone()]
                }
            },
        )
        .last();
        (graph, labels)
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

    fn run_to_completion(&mut self) -> bool {
        while !self.pending.is_empty() {
            let mut new_pending = self
                .pending
                .iter()
                .filter_map(|(_, gate)| gate.apply(&mut self.values))
                .collect::<BTreeMap<_, _>>();
            if new_pending.len() == self.pending.len() {
                return false;
            }
            std::mem::swap(&mut self.pending, &mut new_pending);
        }
        true
    }

    fn gate_for(&self, var: &str) -> Gate {
        self.pending.get(var).unwrap().clone()
    }

    fn outputs_for(&self, in1: &str, in2: &str) -> Vec<Gate> {
        self.pending.iter().filter(|(_, gate)| gate.args().a == in1 && gate.args().b == in2 || gate.args().b == in1 && gate.args().a == in2).map(|(_,g)| g.clone()).collect()
    }

    fn swapped_outputs_for(&self, o1: &str, o2: &str) -> Self {
        Self {
            values: self.values.clone(),
            pending: self
                .pending
                .iter()
                .map(|(o, g)| {
                    if o == o1 {
                        (o.clone(), self.pending.get(o2).unwrap().with_new_output(o))
                    } else if o == o2 {
                        (o.clone(), self.pending.get(o1).unwrap().with_new_output(o))
                    } else {
                        (o.clone(), g.clone())
                    }
                })
                .collect(),
        }
    }

    fn swapped_output_pairs(&self, pairs: &BTreeMap<String, String>) -> Self {
        Self {
            values: self.values.clone(),
            pending: self
                .pending
                .iter()
                .map(|(o, g)| match pairs.get(o.as_str()) {
                    None => (o.clone(), g.clone()),
                    Some(sub) => (
                        o.clone(),
                        self.pending.get(sub.as_str()).unwrap().with_new_output(o),
                    ),
                })
                .collect(),
        }
    }

    fn ancestors_of(&self, var: &str) -> BTreeSet<Gate> {
        BfsIter::new(self.gate_for(var), |v| {
            self.pending
                .iter()
                .map(|(_, gate)| gate)
                .filter(|gate| v.has_input(gate.output()))
                .cloned()
                .collect()
        })
        .collect()
    }

    fn bad_zs(&self) -> Option<BTreeSet<String>> {
        let mut test = self.clone();
        let x = test.extract_num_with("x");
        let y = test.extract_num_with("y");
        if test.run_to_completion() {
            let z = test.extract_num_with("z");
            let goal = x + y;
            let wrong = z ^ goal;
            Some(
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
                    .collect(),
            )
        } else {
            None
        }
    }

    fn bad_z_ancestors(&self) -> Vec<Gate> {
        self.bad_zs()
            .unwrap()
            .iter()
            .map(|z| self.ancestors_of(z.as_str()))
            .reduce(|a, b| a.union(&b).cloned().collect::<BTreeSet<_>>())
            .unwrap()
            .iter()
            .cloned()
            .collect_vec()
    }

    fn single_ancestor_bad(&self) -> Gate {
        self.bad_zs()
            .unwrap()
            .iter()
            .map(|z| self.ancestors_of(z.as_str()))
            .find(|a| a.len() == 1)
            .map(|a| a.iter().next().unwrap().clone())
            .unwrap()
    }

    fn z_adder_report(&self, z: &str) {
        let z_gate = self.pending.get(z).unwrap();
        
        let suffix = &z[1..];
        let x = format!("x{suffix}");
        let y = format!("y{suffix}");
        let xys = self.outputs_for(x.as_str(), y.as_str());
        let mut carry_xy = None;
        let mut xyz = None;
        for xy in xys.iter() {
            if z_gate.has_input(xy.output()) {
                if xyz.is_some() {
                    println!("xyz conflict!");
                }
                xyz = Some(xy.clone());
                println!("XOR {xy}");
            } else {
                if carry_xy.is_some() {
                    println!("carry_xy conflict!");
                }
                carry_xy = Some(xy.clone());
                println!("AND {xy}");
            }
        }

        let mut other_z_in = None;
        if let Some(xyz) = xyz {
            if z_gate.args().a == xyz.output() {
                other_z_in = Some(z_gate.args().b.clone());
            } else if z_gate.args().b == xyz.output() {
                other_z_in = Some(z_gate.args().a.clone());
            }
        }

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

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} -> {}", self.args().a, self.type_name(), self.args().b, self.args().c)
    }
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

    fn apply(&self, values: &mut BTreeMap<String, u128>) -> Option<(String, Self)> {
        self.ops(values)
            .map(|(a, b, c)| {
                values.insert(c, self.eval(a, b));
            })
            .map_or(Some((self.output().to_string(), self.clone())), |_| None)
    }

    fn type_name(&self) -> String {
        match self {
            Gate::And(_) => format!("AND"),
            Gate::Or(_) => format!("OR"),
            Gate::Xor(_) => format!("XOR"),
        }
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
    test.run_to_completion();
    let z = test.extract_num_with("z");
    let goal = x + y;
    let wrong = z ^ goal;
    println!("{wrong:#b}");
    println!("{:?}", circuit.bad_zs());

    let union = circuit.bad_z_ancestors();
    println!(
        "union size: {} ({} gates total)",
        union.len(),
        circuit.pending.len()
    );

    ancestor_analysis(&circuit);
    let for_sure = circuit.single_ancestor_bad();
    for gate in circuit
        .bad_zs()
        .unwrap()
        .iter()
        .filter(|v| for_sure.args().c != **v)
    {
        let alternative = circuit.swapped_outputs_for(for_sure.output(), gate.as_str());
        println!("vs {gate}:");
        ancestor_analysis(&alternative);
    }
}

fn ancestor_analysis(circuit: &Circuit) {
    for z in circuit.bad_zs().unwrap().iter() {
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
    for bad_z in circuit.bad_zs().unwrap() {
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
    let all_outputs = circuit
        .pending
        .iter()
        .map(|(out, _)| out.to_string())
        .collect_vec();

    circuit.run_to_completion();
    let ones = all_singles
        .iter()
        .filter(|s| *circuit.values.get(s.output()).unwrap() == 1)
        .count();
    println!("Singles: Ones: {ones} Zeros: {}", all_singles.len() - ones);

    let ones = all_outputs
        .iter()
        .filter(|o| *circuit.values.get(o.as_str()).unwrap() == 1)
        .count();
    println!("All: Ones: {ones} Zeros: {}", all_outputs.len() - ones);
}
