use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::Display,
    iter::repeat,
};

use common_macros::b_tree_set;
use hash_histogram::HashHistogram;
use itertools::Itertools;

use std::io::Write;

use crate::search_iter::BfsIter;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct AdjacencySets {
    graph: BTreeMap<String, BTreeSet<String>>,
}

impl AdjacencySets {
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.graph.keys().map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn num_edges(&self) -> usize {
        self.pairs().count()
    }

    pub fn num_symmetric_edges(&self) -> usize {
        self.pairs()
            .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
            .collect::<BTreeSet<_>>()
            .len()
    }

    pub fn is_directed(&self) -> bool {
        self.num_symmetric_edges() * 2 != self.num_edges()
    }

    pub fn pairs(&self) -> impl Iterator<Item = (&str, &str)> {
        // Inspired by: https://stackoverflow.com/a/78248495/906268
        self.graph.keys().flat_map(|k| {
            repeat(k.as_str()).zip(
                self.graph
                    .get(k.as_str())
                    .unwrap()
                    .iter()
                    .map(|s| s.as_str()),
            )
        })
    }

    pub fn graphviz(&self, filename: &str) -> anyhow::Result<()> {
        if self.is_directed() {
            graphviz_directed(self.pairs(), filename)
        } else {
            graphviz_undirected(self.pairs(), filename)
        }
    }

    pub fn neighbors_of(&self, node: &str) -> impl Iterator<Item = &str> {
        self.graph.get(node).unwrap().iter().map(|s| s.as_str())
    }

    pub fn are_connected(&self, start: &str, end: &str) -> bool {
        self.graph.get(start).map_or(false, |set| set.contains(end))
    }

    pub fn connect2(&mut self, start: &str, end: &str) {
        self.connect(start, end);
        self.connect(end, start);
    }

    pub fn connect(&mut self, start: &str, end: &str) {
        match self.graph.get_mut(start) {
            None => {
                self.graph
                    .insert(start.to_string(), b_tree_set! {end.to_string()});
            }
            Some(connections) => {
                connections.insert(end.to_string());
            }
        }
        if !self.graph.contains_key(end) {
            self.graph.insert(end.to_string(), BTreeSet::new());
        }
    }

    pub fn in_degrees(&self) -> HashMap<String, usize> {
        self.degrees(
            self.pairs()
                .map(|(_, dest)| dest)
                .collect::<HashHistogram<_>>(),
        )
    }

    pub fn out_degrees(&self) -> HashMap<String, usize> {
        self.degrees(
            self.pairs()
                .map(|(src, _)| src)
                .collect::<HashHistogram<_>>(),
        )
    }

    fn degrees(&self, above_zero: HashHistogram<&str>) -> HashMap<String, usize> {
        let mut result = above_zero
            .iter()
            .map(|(s, c)| (s.to_string(), *c))
            .collect::<HashMap<_, _>>();
        for v in self.graph.keys() {
            if !result.contains_key(v) {
                result.insert(v.clone(), 0);
            }
        }
        result
    }

    // Kahn's Algorithm
    // Returns None if the graph contains a cycle.
    pub fn topologial_ordering(&self) -> Option<Vec<String>> {
        let mut in_degrees = self.in_degrees();
        let source_nodes = in_degrees
            .iter()
            .filter(|(_, c)| **c == 0)
            .map(|(n, _)| n.clone())
            .collect_vec();
        let visited = BfsIter::multi_start(source_nodes.iter().cloned(), |n| {
            let mut next = vec![];
            println!("visiting {n}");
            for neighbor in self.neighbors_of(n.as_str()) {
                let count = in_degrees.get_mut(neighbor).unwrap();
                *count -= 1;
                if *count == 0 {
                    next.push(neighbor.to_string());
                }
            }
            next
        })
        .collect_vec();
        if visited.len() == self.len() {
            Some(visited)
        } else {
            None
        }
    }
}

// graphviz:
// To view the generated file, use:
//   dot -Tpng -Kneato -O [filename]
pub fn graphviz_undirected<N: Display, I: Iterator<Item = (N, N)>>(
    items: I,
    output_filename: &str,
) -> anyhow::Result<()> {
    graphviz(items, output_filename, "graph", "--")
}

pub fn graphviz_directed<N: Display, I: Iterator<Item = (N, N)>>(
    items: I,
    output_filename: &str,
) -> anyhow::Result<()> {
    graphviz(items, output_filename, "digraph", "->")
}

fn graphviz<N: Display, I: Iterator<Item = (N, N)>>(
    items: I,
    output_filename: &str,
    header: &str,
    edge: &str,
) -> anyhow::Result<()> {
    let mut file_out = std::fs::File::create(output_filename)?;
    writeln!(file_out, "{header} G {{")?;
    for (src, dest) in items {
        writeln!(file_out, "  {src} {edge} {dest}")?;
    }
    writeln!(file_out, "}}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{graph::AdjacencySets, search_iter::BfsIter};

    // TODO: Need more tests for the following:
    // * Directed graph example

    #[test]
    fn graph_test() {
        let mut graph = AdjacencySets::default();
        for (a, b) in [
            ("start", "A"),
            ("start", "b"),
            ("A", "c"),
            ("A", "b"),
            ("b", "d"),
            ("A", "end"),
            ("b", "end"),
        ] {
            graph.connect2(a, b);
        }
        assert_eq!(graph.num_edges(), 14);
        assert_eq!(graph.num_symmetric_edges(), 7);
        assert!(!graph.is_directed());

        let keys = graph.keys().collect::<Vec<_>>();
        assert_eq!(keys, vec!["A", "b", "c", "d", "end", "start"]);
        let mut searcher = BfsIter::new("start", |s| graph.neighbors_of(s).collect());
        let found = searcher.by_ref().collect_vec();
        assert_eq!(found, vec!["start", "A", "b", "c", "end", "d"]);

        let path = searcher.path_back_from(&"end");
        let path_str = format!("{:?}", path);
        assert_eq!(path_str, r#"["end", "A", "start"]"#);
    }

    #[test]
    fn test_pair_iter() {
        let mut graph = AdjacencySets::default();
        for (a, b) in [
            ("start", "A"),
            ("start", "b"),
            ("A", "c"),
            ("A", "b"),
            ("b", "d"),
            ("A", "end"),
            ("b", "end"),
        ] {
            graph.connect2(a, b);
        }

        let pair_str = format!("{:?}", graph.pairs().collect_vec());
        assert_eq!(
            pair_str.as_str(),
            r#"[("A", "b"), ("A", "c"), ("A", "end"), ("A", "start"), ("b", "A"), ("b", "d"), ("b", "end"), ("b", "start"), ("c", "A"), ("d", "b"), ("end", "A"), ("end", "b"), ("start", "A"), ("start", "b")]"#
        );
    }
}
