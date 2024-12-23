use std::collections::BTreeSet;

use advent2024::{advent_main, all_lines, graph::AdjacencySets, Part};
use common_macros::b_tree_set;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let mut graph = AdjacencySets::default();
        for line in all_lines(filename)? {
            let (start, end) = line.split('-').collect_tuple().unwrap();
            graph.connect2(start, end);
        }

        if options.contains(&"-size") {
            println!("nodes: {}", graph.len());
            println!("edges: {}", graph.num_edges());
        }

        match part {
            Part::One => {
                let t_cliques = clique3(&graph).iter().filter(|c| c.iter().any(|s| s.starts_with("t"))).count();
                println!("{t_cliques}");
            }
            Part::Two => {
                let mut biggest: Option<BTreeSet<&str>> = None;
                for mut clique in clique3(&graph) {
                    for node in graph.keys() {
                        if !clique.contains(node) && clique.iter().all(|cn| graph.are_connected(*&cn, node)) {
                            clique.insert(node);
                        }
                    }
                    if biggest.as_ref().map_or(true, |b| clique.len() > b.len()) {
                        biggest = Some(clique);
                    }
                }
                let result = biggest.unwrap().iter().join(",");
                println!("{result}");
            }
        }
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
