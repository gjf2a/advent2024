use common_macros::hash_map;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;
use trait_set::trait_set;
use num::Integer;

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

trait_set! {
    pub trait Estimator = Integer + Copy + Clone + Add<Output=Self> + PartialOrd + Ord + Debug + Default
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Ord)]
struct TotalEstimate<N: Estimator> {
    from_start: N,
    estimate_to_goal: N,
}

impl<N: Estimator> TotalEstimate<N> {
    fn new(from_start: N, estimate_to_goal: N) -> Self {
        Self {from_start, estimate_to_goal}
    }

    fn total(&self) -> N {
        self.from_start + self.estimate_to_goal
    }
}

impl<N: Estimator> PartialOrd for TotalEstimate<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.total().partial_cmp(&self.total())
    }
}

pub struct PrioritySearchIter<
N: Estimator,
    T: SearchNode,
    I: Iterator<Item = T>,
> {
    queue: PriorityQueue<T, TotalEstimate<N>>,
    costs: HashMap<T, N>,
    parents: HashMap<T, Option<T>>,
    successor: fn(T) -> I,
    cost: fn(&T) -> N,
    heuristic: fn(&T) -> N,
}

impl<N: Estimator, T: SearchNode, I: Iterator<Item = T>>
    PrioritySearchIter<N, T, I>
{
    pub fn a_star(start: T, successor: fn(T) -> I, cost: fn(&T) -> N, heuristic: fn(&T) -> N) -> Self {
        let mut queue = PriorityQueue::new();
        queue.push(start.clone(), TotalEstimate::default());
        Self {
            queue,
            costs: hash_map!(start.clone() => N::zero()),
            successor,
            parents: hash_map!(start.clone() => None),
            cost,
            heuristic,
        }
    }

    pub fn dijkstra(start: T, successor: fn(T) -> I, cost: fn(&T) -> N) -> Self {
        Self::a_star(start, successor, cost, |_| N::zero())
    }

    pub fn path_back_from(&self, node: &T) -> VecDeque<T> {
        path_back_from(node, &self.parents)
    }

    pub fn cost_for(&self, node: &T) -> N {
        self.costs.get(node).copied().unwrap()
    }
}

impl<T: SearchNode, I: Iterator<Item = T>> PrioritySearchIter<usize, T, I> {
    pub fn bfs(start: T, successor: fn(T) -> I) -> Self {
        Self::dijkstra(start, successor, |_| 1)
    }
}

impl<N: Estimator, T: SearchNode, I: Iterator<Item = T>> Iterator
    for PrioritySearchIter<N, T, I>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop().map(|(parent, cost)| {
            self.costs.insert(parent.clone(), cost.from_start);
            for child in (self.successor)(parent.clone()) {
                if !self.costs.contains_key(&child) {
                    let new_priority = TotalEstimate::new(cost.from_start + (self.cost)(&child), (self.heuristic)(&child));
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
        search_iter::{path_back_from, BfsIter, PrioritySearchIter},
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

    #[test]
    fn test_priority_bfs() {
        println!("Test Priority BFS");
        let max_dist = 2;
        let start = (Position::default(), Position::default(), max_dist);
        println!("Starting BFS");
        let mut searcher = PrioritySearchIter::bfs(start, |n| {
            all::<ManhattanDir>()
                .map(move |d| (d.neighbor(n.0), n.1, n.2))
                .filter(|(p,start,m)| p.manhattan_distance(start) <= *m)
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
