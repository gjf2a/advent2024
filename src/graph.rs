use std::{
    collections::{BTreeMap, BTreeSet},
    iter::repeat,
};

use common_macros::b_tree_set;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdjacencySets {
    graph: BTreeMap<String, BTreeSet<String>>,
}

impl AdjacencySets {
    pub fn new() -> Self {
        AdjacencySets {
            graph: BTreeMap::new(),
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.graph.keys().map(|s| s.as_str())
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
            }
            Some(connections) => {
                connections.insert(end.to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::AdjacencySets,
        searchers::{breadth_first_search, ContinueSearch, SearchQueue},
    };
/* 
    #[test]
    fn graph_test() {
        let mut graph = AdjacencySets::new();
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

        let parent_map = breadth_first_search("start", |node, q| {
            graph
                .neighbors_of(node)
                .for_each(|n| q.enqueue(n));
            ContinueSearch::Yes
        });
        let parent_map_str = format!("{:?}", parent_map);
        assert_eq!(
            parent_map_str.as_str(),
            r#"ParentMap { parents: {"start": None, "A": Some("start"), "b": Some("start"), "c": Some("A"), "end": Some("A"), "d": Some("b")}, last_dequeued: Some("d") }"#
        );
        let path = parent_map.path_back_from(&"end".to_string()).unwrap();
        let path_str = format!("{:?}", path);
        assert_eq!(path_str, r#"["start", "A", "end"]"#);
    }
*/
    #[test]
    fn test_pair_iter() {
        let mut graph = AdjacencySets::new();
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

        for (k, v) in graph.pairs() {
            println!("{k} {v}");
        }
    }
}
