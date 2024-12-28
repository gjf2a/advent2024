use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    str::FromStr,
    time::Instant,
};

use advent2024::{
    advent_main, all_lines,
    graph::{graphviz_directed, AdjacencySets},
    search_iter::BfsIter,
    Part,
};
use anyhow::anyhow;
use common_macros::b_tree_map;
use hash_histogram::HashHistogram;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let circuit = Circuit::from_file(filename)?;
        if options.contains(&"-showzs") {
            show_bad_zs(circuit);
        } else if options.contains(&"-singles") {
            show_single_ancestors(circuit);
        } else if options.contains(&"-swapall") {
            swap_every_pair(circuit);
        } else if options.contains(&"-dot") {
            let graph = circuit.directed_edges();
            graphviz_directed(graph.iter().cloned(), "day24.dot")?;
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
    for (src, dest) in circuit.directed_edges() {
        graph.connect(src.as_str(), dest.as_str());
    }
    let graph = graph;
    let in_degrees = graph.in_degrees();
    let mut best_circuit = circuit.clone();
    let mut best_zs = circuit.bad_zs().unwrap();
    let mut swapped = BTreeSet::new();
    let topo = graph.topologial_ordering().unwrap();
    let topo_output = topo
        .iter()
        .filter(|s| *(in_degrees.get(*s).unwrap()) == 2)
        .cloned()
        .collect_vec();
    for i in 0..topo_output.len() {
        println!("{}: {}/{}", topo_output[i], (i + 1), topo_output.len());
        for j in (i + 1)..topo_output.len() {
            if !swapped.contains(topo_output[i].as_str()) {
                if !swapped.contains(topo_output[j].as_str()) {
                    //println!("\t{}: {}/{}", topo_output[j], (j + 1), topo_output.len());
                    let test =
                        best_circuit.swapped_outputs_for(topo_output[i].as_str(), topo_output[j].as_str());
                    if let Some(test_zs) = test.bad_zs() {
                        if test_zs.len() < best_zs.len() {
                            best_circuit = test;
                            best_zs = test_zs;
                            println!("swapping {} and {} ({} bad zs)", topo_output[i], topo_output[j], best_zs.len());
                            swapped.insert(topo_output[i].to_string());
                            swapped.insert(topo_output[j].to_string());
                        }
                    }
                }
            }

        }
    }

    let output = swapped.iter().join(",");
    println!("{output}");
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

    fn directed_edges(&self) -> Vec<(String, String)> {
        let mut graph = vec![];
        BfsIter::multi_start(
            self.pending.keys().cloned().filter(|k| k.starts_with("z")),
            |output| match self.pending.get(output) {
                None => vec![],
                Some(gate) => {
                    graph.push((gate.args().a.clone(), output.clone()));
                    graph.push((gate.args().b.clone(), output.clone()));
                    vec![gate.args().a.clone(), gate.args().b.clone()]
                }
            },
        )
        .last();
        graph
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

    fn swapped_output_pairs(&self, pairs: &BTreeMap<&&str, &&str>) -> Self {
        Self {
            values: self.values.clone(),
            pending: self
                .pending
                .iter()
                .map(|(o, g)| match pairs.get(&o.as_str()) {
                    None => (o.clone(), g.clone()),
                    Some(sub) => (
                        o.clone(),
                        self.pending.get(**sub).unwrap().with_new_output(o),
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

    fn apply(&self, values: &mut BTreeMap<String, u128>) -> Option<(String, Self)> {
        self.ops(values)
            .map(|(a, b, c)| {
                values.insert(c, self.eval(a, b));
            })
            .map_or(Some((self.output().to_string(), self.clone())), |_| None)
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

fn swap_every_pair(circuit: Circuit) {
    let bad_zs = circuit.bad_zs().unwrap();
    let mut bad_z_options = bad_zs
        .iter()
        .map(|z| (z, vec![]))
        .collect::<HashMap<_, _>>();
    let all_ancestors = circuit.bad_z_ancestors();

    let mut improvements = HashHistogram::<usize>::new();
    let mut options = HashMap::new();
    let mut swappees = AdjacencySets::default();
    for i in 0..all_ancestors.len() {
        for j in (i + 1)..all_ancestors.len() {
            let o1 = all_ancestors[i].output();
            let o2 = all_ancestors[j].output();
            let alternative = circuit.swapped_outputs_for(o1, o2);
            if let Some(swapped_zs) = alternative.bad_zs() {
                if swapped_zs.iter().all(|z| bad_zs.contains(z)) && swapped_zs.len() < bad_zs.len()
                {
                    for z in swapped_zs.iter() {
                        bad_z_options.get_mut(z).unwrap().push((o1, o2));
                    }
                    let remaining_zs = bad_zs
                        .iter()
                        .filter(|z| !swapped_zs.contains(z.as_str()))
                        .collect_vec();
                    let improvement = bad_zs.len() - swapped_zs.len();
                    improvements.bump(&improvement);
                    options.insert((o1, o2), remaining_zs);
                    swappees.connect2(o1, o2);
                }
            }
        }
    }
    let mut compatible = HashMap::new();
    for ((a, b), zs) in options.iter() {
        compatible.insert((a, b), vec![]);
        for ((a2, b2), zs2) in options.iter() {
            if a != a2 && b != b2 && zs.iter().all(|z| !zs2.contains(z)) {
                compatible.get_mut(&(a, b)).unwrap().push((a2, b2));
            }
        }
    }
    println!("improved: {:?}", improvements.ranking_with_counts());
    let most_alternatives = compatible.values().map(|v| v.len()).max().unwrap();
    let least_alternatives = compatible.values().map(|v| v.len()).min().unwrap();
    println!("most alternatives: {most_alternatives} (least: {least_alternatives})");

    let start = Instant::now();
    for (opt, ((a, b), zs)) in options.iter().enumerate() {
        let elapsed = Instant::now().duration_since(start).as_secs_f32();
        println!("Considering option {opt} / {} ({elapsed}s)", options.len());
        let zs = zs.iter().collect::<BTreeSet<_>>();
        let swaps = b_tree_map![a => b, b => a];
        let compat = compatible.get(&(a, b)).unwrap();
        for i in 0..compat.len() {
            let mut swaps_i = swaps.clone();
            let (ai, bi) = compat[i];
            swaps_i.insert(ai, bi);
            swaps_i.insert(bi, ai);
            let mut zs_i = zs.clone();
            for z in options.get(&(ai, bi)).unwrap().iter() {
                zs_i.insert(z);
            }
            for j in (i + 1)..compat.len() {
                let (aj, bj) = compat[j];
                if !swaps_i.contains_key(aj) && !swaps_i.contains_key(bj) {
                    let mut swaps_j = swaps_i.clone();
                    swaps_j.insert(aj, bj);
                    swaps_j.insert(bj, aj);
                    let mut zs_j = zs_i.clone();
                    for z in options.get(&(aj, bj)).unwrap().iter() {
                        zs_j.insert(z);
                    }
                    for k in (j + 1)..compat.len() {
                        let (ak, bk) = compat[k];
                        if !swaps_j.contains_key(ak) && !swaps_j.contains_key(bk) {
                            let mut swaps_k = swaps_j.clone();
                            swaps_k.insert(ak, bk);
                            swaps_k.insert(bk, ak);
                            let mut zs_k = zs_j.clone();
                            for z in options.get(&(ak, bk)).unwrap().iter() {
                                zs_k.insert(z);
                            }
                            if zs_k.len() == bad_zs.len() {
                                let mut test = circuit.swapped_output_pairs(&swaps);
                                let x = test.extract_num_with("x");
                                let y = test.extract_num_with("y");
                                test.run_to_completion();
                                let z = test.extract_num_with("z");
                                let goal = x + y;
                                let wrong = z ^ goal;
                                if wrong == 0 {
                                    println!("We have a winner!");
                                    println!("{}", swaps_k.keys().join(","));
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
