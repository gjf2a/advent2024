use std::collections::BTreeSet;

use advent2024::{advent_main, all_lines, graph::AdjacencySets};
use common_macros::b_tree_set;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, _part, _| {
        let mut graph = AdjacencySets::new();
        for line in all_lines(filename)? {
            let (start, end) = line.split('-').collect_tuple().unwrap();
            graph.connect2(start, end);
        }
        let t_cliques = clique3(&graph).iter().filter(|c| c.iter().any(|s| s.starts_with("t"))).count();
        println!("{t_cliques}");
        Ok(())
    })
}

fn clique3(graph: &AdjacencySets) -> BTreeSet<BTreeSet<&str>> {
    let mut result = BTreeSet::new();
    for (a, b) in graph.pairs() {
        for c in graph.neighbors_of(b).filter(|n| graph.are_connected(a, *n)) {
            result.insert(b_tree_set! {a, b, c});
        }
    }
    result
}
