use common_macros::hash_map;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use trait_set::trait_set;

trait_set! {
    pub trait SearchNode = Clone + Hash + Eq + Debug;
}

pub struct BfsIter<T: SearchNode, S: Fn(T) -> I, I: Iterator<Item = T>> {
    queue: VecDeque<(T, usize)>,
    depths: HashMap<T, usize>,
    parents: HashMap<T, Option<T>>,
    successor: S,
}

impl<T: SearchNode, S: Fn(T) -> I, I: Iterator<Item = T>> BfsIter<T, S, I> {
    pub fn new(start: T, successor: S) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back((start.clone(), 0));
        Self {
            queue,
            depths: hash_map!(start.clone() => 0),
            successor,
            parents: hash_map!(start.clone() => None),
        }
    }

    pub fn path_back_from(&self, node: &T) -> VecDeque<T> {
        path_back_from(node, &self.parents)
    }

    pub fn depth_for(&self, node: &T) -> usize {
        self.depths.get(node).copied().unwrap()
    }
}

impl<T: SearchNode, S: Fn(T) -> I, I: Iterator<Item = T>> Iterator for BfsIter<T, S, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().map(|(parent, depth)| {
            for child in (self.successor)(parent.clone()) {
                if !self.depths.contains_key(&child) {
                    self.depths.insert(child.clone(), depth + 1);
                    self.parents.insert(child.clone(), Some(parent.clone()));
                    self.queue.push_back((child, depth + 1));
                }
            }
            parent
        })
    }
}

fn path_back_from<T: SearchNode>(node: &T, parents: &HashMap<T, Option<T>>) -> VecDeque<T> {
    let mut result = VecDeque::new();
    let mut current = node;
    result.push_back(current.clone());
    while let Some(parent) = parents.get(current).unwrap() {
        result.push_back(parent.clone());
        current = parent;
    }
    result
}

pub struct PrioritySearchIter<
    T: SearchNode,
    S: Fn(T) -> I,
    I: Iterator<Item = T>,
    C: Fn(&T) -> usize,
> {
    queue: PriorityQueue<T, Reverse<usize>>,
    costs: HashMap<T, usize>,
    parents: HashMap<T, Option<T>>,
    successor: S,
    cost: C,
}

impl<T: SearchNode, S: Fn(T) -> I, I: Iterator<Item = T>, C: Fn(&T) -> usize>
    PrioritySearchIter<T, S, I, C>
{
    pub fn new(start: T, successor: S, cost: C) -> Self {
        let mut queue = PriorityQueue::new();
        queue.push(start.clone(), Reverse(0));
        Self {
            queue,
            costs: hash_map!(start.clone() => 0),
            successor,
            parents: hash_map!(start.clone() => None),
            cost,
        }
    }

    pub fn path_back_from(&self, node: &T) -> VecDeque<T> {
        path_back_from(node, &self.parents)
    }

    pub fn cost_for(&self, node: &T) -> usize {
        self.costs.get(node).copied().unwrap()
    }
}

impl<T: SearchNode, S: Fn(T) -> I, I: Iterator<Item = T>, C: Fn(&T) -> usize> Iterator
    for PrioritySearchIter<T, S, I, C>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop().map(|(parent, cost)| {
            self.costs.insert(parent.clone(), cost.0);
            for child in (self.successor)(parent.clone()) {
                let new_priority = Reverse((self.cost)(&child) + cost.0);
                match self.queue.get_priority(&child) {
                    Some(priority) => {
                        if new_priority > *priority {
                            self.parents.insert(child.clone(), Some(parent.clone()));
                            self.queue.change_priority(&child, new_priority);
                        }
                    }
                    None => {
                        self.parents.insert(child.clone(), Some(parent.clone()));
                        self.queue.push(child, new_priority);
                    }
                }
            }
            parent
        })
    }
}

#[cfg(test)]
mod tests {
    use enum_iterator::all;

    use crate::{
        multidim::{DirType, ManhattanDir, Position},
        search_iter::{path_back_from, BfsIter},
    };

    #[test]
    fn test_bfs() {
        println!("Test BFS");
        let max_dist = 2;
        let start = Position::default();
        println!("Starting BFS");
        let mut searcher = BfsIter::new(start, |n| {
            all::<ManhattanDir>()
                .map(move |d| d.neighbor(n))
                .filter(|p| start.manhattan_distance(p) <= max_dist)
        });
        searcher.by_ref().last();
        println!("Search complete.");
        assert_eq!(searcher.parents.len(), 13);

        for node in searcher.parents.keys() {
            let len = path_back_from(node, &searcher.parents).len();
            println!("From {:?}: {}", node, len);
            assert!(len <= 1 + max_dist as usize);
        }
    }
}
