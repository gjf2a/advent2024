use std::{cmp::min, collections::{HashMap, HashSet}};

use advent2024::{
    advent_main,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    searchers::{breadth_first_search, ContinueSearch, SearchQueue},
    Part,
};
use enum_iterator::all;
use hash_histogram::HashHistogram;

// Part 1: New version works. Still wondering about why Labeler is incorrect.
// Part 2: 849746 is too low. 887098 is too high. 858973 is too low.

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let garden = GridCharWorld::from_char_file(filename)?;
        let (points2regions, first_found) = bfs_points2regions(&garden);
        let regions = region2chars(&garden, &points2regions);
        let areas = region2areas(&points2regions);
        let perimeters = match part {
            Part::One => perimeter1(&points2regions),
            Part::Two => perimeter2(&garden, &first_found, &regions, &points2regions),
        };
        let total = regions
            .keys()
            .map(|region| areas.count(&region) * perimeters.count(&region))
            .sum::<usize>();
        println!("{total}");
        Ok(())
    })
}

fn region2areas(points2regions: &HashMap<Position, usize>) -> HashHistogram<usize> {
    points2regions.values().collect()
}

fn region2chars(
    garden: &GridCharWorld,
    points2regions: &HashMap<Position, usize>,
) -> HashMap<usize, char> {
    let mut result = HashMap::new();
    for (p, r) in points2regions.iter() {
        if !result.contains_key(r) {
            result.insert(*r, garden.value(*p).unwrap());
        }
    }
    result
}

fn perimeter1(points2regions: &HashMap<Position, usize>) -> HashHistogram<usize> {
    let mut perimeters = HashHistogram::new();
    for (p, region) in points2regions.iter() {
        perimeters.bump_by(region, edge_count(*p, &points2regions));
    }
    perimeters
}

fn perimeter2(
    garden: &GridCharWorld,
    first_found: &HashMap<usize, Position>,
    regions: &HashMap<usize, char>,
    points2regions: &HashMap<Position, usize>,
) -> HashHistogram<usize> {
    let mut garden_copy = garden.clone();
    let mut result = HashHistogram::new();
    let mut inside_sides = HashHistogram::new();
    for (region, start) in first_found.iter() {
        let start = EdgeFollower::new(*regions.get(region).unwrap(), *start);
        let mut neighboring_regions = HashSet::new();
        let mut explorer = start;
        loop {
            garden_copy.update(explorer.p, garden.value(explorer.p).unwrap().to_ascii_lowercase());
            if let Some(outside_right) = explorer.outside_to_right(garden) {
                if let Some(outside_region) = points2regions.get(&outside_right) {
                    neighboring_regions.insert(*outside_region);
                }
                if explorer.blocked_ahead(garden) {
                    explorer.left();
                    result.bump(region);
                } else {
                    explorer.forward();
                }
            } else {
                explorer.right();
                explorer.forward();
                result.bump(region);
            }
            if explorer == start {
                if neighboring_regions.len() == 1 {
                    let surrounder = neighboring_regions.iter().next().unwrap();
                    inside_sides.bump_by(surrounder, result.count(region));
                }
                break;
            }
        }
    }
    for (region, inside_count) in inside_sides.iter() {
        result.bump_by(region, *inside_count);
    }
    println!("{garden_copy}");
    result
}

fn edges(p: Position, points2regions: &HashMap<Position, usize>) -> Vec<ManhattanDir> {
    let region = points2regions.get(&p).unwrap();
    all::<ManhattanDir>()
        .filter(|d| {
            points2regions
                .get(&d.neighbor(p))
                .map_or(true, |r| r != region)
        })
        .collect()
}

fn edge_count(p: Position, points2regions: &HashMap<Position, usize>) -> usize {
    edges(p, points2regions).len()
}

fn bfs_points2regions(
    garden: &GridCharWorld,
) -> (HashMap<Position, usize>, HashMap<usize, Position>) {
    let mut current = 0;
    let mut result = HashMap::new();
    let mut first_found = HashMap::new();
    for (p, v) in garden.position_value_iter() {
        if !result.contains_key(p) {
            breadth_first_search(p, |s, q| {
                if !first_found.contains_key(&current) {
                    first_found.insert(current, *s);
                }
                result.insert(*s, current);
                for d in all::<ManhattanDir>() {
                    let n = d.neighbor(*s);
                    if garden.value(n).map_or(false, |c| c == *v) {
                        q.enqueue(&n);
                    }
                }
                ContinueSearch::Yes
            });
            current += 1;
        }
    }
    (result, first_found)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct EdgeFollower {
    c: char,
    p: Position,
    f: ManhattanDir,
}

impl EdgeFollower {
    fn new(c: char, p: Position) -> Self {
        EdgeFollower {
            c,
            p,
            f: ManhattanDir::S,
        }
    }

    fn outside_neighbor(&self, world: &GridCharWorld, dir: ManhattanDir) -> Option<Position> {
        let neighbor = dir.neighbor(self.p);
        match world.value(neighbor) {
            None => Some(neighbor),
            Some(c) => if self.c == c {None} else {Some(neighbor)},
        }
    }

    fn neighbor_in_region(&self, world: &GridCharWorld, dir: ManhattanDir) -> bool {
        world
            .value(dir.neighbor(self.p))
            .map_or(false, |c| self.c == c)
    }

    fn outside_to_right(&self, world: &GridCharWorld) -> Option<Position> {
        self.outside_neighbor(world, self.f.clockwise())
    }

    fn blocked_ahead(&self, world: &GridCharWorld) -> bool {
        !self.neighbor_in_region(world, self.f)
    }

    fn forward(&mut self) {
        self.p = self.f.neighbor(self.p);
    }

    fn left(&mut self) {
        self.f = self.f.counterclockwise();
    }

    fn right(&mut self) {
        self.f = self.f.clockwise();
    }
}

// I want to test this against my BFS and figure out my mistake.
#[allow(dead_code)]
fn points2regions(garden: &GridCharWorld) -> HashMap<Position, usize> {
    let mut result = HashMap::new();
    let mut equivalencies = Labeler::default();
    for (p, v) in garden.position_value_iter() {
        let n_char_match = char_match_label(*v, ManhattanDir::N.neighbor(*p), garden, &result);
        let w_char_match = char_match_label(*v, ManhattanDir::W.neighbor(*p), garden, &result);
        let label = match w_char_match {
            None => match n_char_match {
                None => equivalencies.new_label(),
                Some(l) => l,
            },
            Some(wl) => {
                if let Some(nl) = n_char_match {
                    equivalencies.mark_equal(nl, wl);
                }
                wl
            }
        };
        result.insert(*p, label);
    }
    result
        .iter()
        .map(|(k, v)| (*k, equivalencies.get(*v)))
        .collect()
}

fn char_match_label(
    c: char,
    n: Position,
    garden: &GridCharWorld,
    labels: &HashMap<Position, usize>,
) -> Option<usize> {
    garden
        .value(n)
        .filter(|nc| *nc == c)
        .map(|_| labels.get(&n).copied().unwrap())
}

#[derive(Clone, Default)]
struct Labeler {
    equivalencies: Vec<usize>,
}

impl Labeler {
    fn new_label(&mut self) -> usize {
        let result = self.equivalencies.len();
        self.equivalencies.push(result);
        result
    }

    fn mark_equal(&mut self, label1: usize, label2: usize) {
        let keep = min(self.get(label1), self.get(label2));
        self.equivalencies[label1] = keep;
        self.equivalencies[label2] = keep;
    }

    fn get(&self, label: usize) -> usize {
        if self.equivalencies[label] == label {
            label
        } else {
            self.get(self.equivalencies[label])
        }
    }
}
