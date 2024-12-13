use std::{cmp::min, collections::HashMap};

use advent2024::{
    advent_main,
    grid::GridCharWorld,
    multidim::{Dir, DirType, ManhattanDir, Position},
    searchers::{breadth_first_search, ContinueSearch, SearchQueue},
    Part,
};
use enum_iterator::all;
use hash_histogram::HashHistogram;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let garden = GridCharWorld::from_char_file(filename)?;
        let points2regions = bfs_points2regions(&garden);
        let regions = region2chars(&garden, &points2regions);
        let areas = region2areas(&points2regions);
        let perimeters = match part {
            Part::One => perimeter1(&points2regions),
            Part::Two => perimeter2(&points2regions),
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

fn perimeter2(points2regions: &HashMap<Position, usize>) -> HashHistogram<usize> {
    let mut sides = HashHistogram::new();
    for (p, region) in points2regions.iter() {
        for dir in all::<ManhattanDir>() {
            let off = dir.clockwise();
            let dir_region = neighbor_region(p, dir, points2regions);
            let off_region = neighbor_region(p, off, points2regions);
            if !regions_eq(Some(*region), off_region) && !regions_eq(Some(*region), dir_region) {
                sides.bump(region);
                if regions_eq(dir_region, off_region) {
                    if let Some(outer_region) = dir_region {
                        let diag = Dir::from(dir).clockwise();
                        if let Some(diag_region) = points2regions.get(&diag.neighbor(*p)) {
                            if *diag_region == outer_region {
                                sides.bump(&outer_region);
                            }
                        }
                    }
                }
            }
        }
    }
    sides
}

fn neighbor_region(
    p: &Position,
    dir: ManhattanDir,
    points2regions: &HashMap<Position, usize>,
) -> Option<usize> {
    let np = dir.neighbor(*p);
    points2regions.get(&np).copied()
}

fn regions_eq(r1: Option<usize>, r2: Option<usize>) -> bool {
    match r1 {
        None => r2.is_none(),
        Some(r1) => match r2 {
            None => false,
            Some(r2) => r1 == r2,
        },
    }
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

fn bfs_points2regions(garden: &GridCharWorld) -> HashMap<Position, usize> {
    let mut current = 0;
    let mut result = HashMap::new();
    for (p, v) in garden.position_value_iter() {
        if !result.contains_key(p) {
            breadth_first_search(p, |s, q| {
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
    result
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
