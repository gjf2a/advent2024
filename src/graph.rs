use std::{
    collections::{BTreeMap, BTreeSet},
    iter::repeat,
};

use common_macros::b_tree_set;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct AdjacencySets {
    graph: BTreeMap<String, BTreeSet<String>>,
    num_edges: usize,
}

impl AdjacencySets {
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.graph.keys().map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn num_edges(&self) -> usize {
        self.num_edges
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
                self.num_edges += 1;
            }
            Some(connections) => {
                let before = connections.len();
                connections.insert(end.to_string());
                if connections.len() > before {
                    self.num_edges += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{
        graph::AdjacencySets, search_iter::BfsIter
    };
    
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
        let keys = graph.keys().collect::<Vec<_>>();
        assert_eq!(keys, vec!["A", "b", "c", "d", "end", "start"]);
        let mut searcher = BfsIter::new("start", |s| graph.neighbors_of(s).collect());
        let found = searcher.by_ref().collect_vec();
        println!("{found:?}");

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
        assert_eq!(pair_str.as_str(), r#"[("A", "b"), ("A", "c"), ("A", "end"), ("A", "start"), ("b", "A"), ("b", "d"), ("b", "end"), ("b", "start"), ("c", "A"), ("d", "b"), ("end", "A"), ("end", "b"), ("start", "A"), ("start", "b")]"#);
    }
}
