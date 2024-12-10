use std::collections::VecDeque;

use advent2024::{
    advent_main,
    grid::GridDigitWorld,
    multidim::{DirType, ManhattanDir, Position},
    searchers::{breadth_first_search, ContinueSearch, SearchQueue},
    Part,
};
use bare_metal_modulo::MNum;
use enum_iterator::all;
use hash_histogram::HashHistogram;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let topomap = GridDigitWorld::from_digit_file(filename)?;
        let mut total = 0;
        for (start, _) in topomap.position_value_iter().filter(|(_, v)| **v == 0) {
            total += match part {
                Part::One => num_reachable_peaks(start, &topomap),
                Part::Two => num_distinct_paths(start, &topomap),
            }
        }
        println!("{total}");
        Ok(())
    })
}

fn num_reachable_peaks(start: &Position, topomap: &GridDigitWorld) -> usize {
    let mut nine_count = 0;
    breadth_first_search(start, |p, q| {
        let height = topomap.value(*p).unwrap();
        if height == 9 {
            nine_count += 1;
        } else {
            for n in ascending_neighbors(*p, topomap) {
                q.enqueue(&n);
            }
        }
        ContinueSearch::Yes
    });
    nine_count
}

fn ascending_neighbors(
    p: Position,
    topomap: &GridDigitWorld,
) -> impl Iterator<Item = Position> + '_ {
    let target_height = topomap.value(p).unwrap().a() + 1;
    all::<ManhattanDir>()
        .map(move |d| d.neighbor(p))
        .filter(move |n| topomap.value(*n).map_or(false, |h| h == target_height))
}

fn num_distinct_paths(start: &Position, topomap: &GridDigitWorld) -> usize {
    let mut num_incoming = HashHistogram::new();
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0));
    while let Some((p, incoming_count)) = queue.pop_front() {
        let seen_before = num_incoming.count(&p) > 0;
        num_incoming.bump_by(&p, incoming_count);
        if !seen_before {
            for n in ascending_neighbors(p, topomap) {
                queue.push_back((n, num_incoming.count(&n)));
            }
        } 
    }
    println!("start: {start}");
    println!("{num_incoming}");
    println!();
    num_incoming.iter().filter(|(p, _)| topomap.value(**p).unwrap() == 9).map(|(_, c)| *c).sum()
}
