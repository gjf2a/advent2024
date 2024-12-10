use std::collections::BTreeMap;

use advent2024::{
    advent_main,
    grid::GridDigitWorld,
    multidim::{DirType, ManhattanDir, Position},
    searchers::{breadth_first_search, ContinueSearch, SearchQueue},
    Part,
};
use bare_metal_modulo::{MNum, ModNumC};
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
    let mut height2locations = BTreeMap::new();
    breadth_first_search(start, |p, q| {
        let height = topomap.value(*p).unwrap();
        //println!("height: {height}");
        match height2locations.get_mut(&height) {
            None => {height2locations.insert(height, vec![*p]);}
            Some(v) => v.push(*p)
        };
        for n in ascending_neighbors(*p, topomap) {
            q.enqueue(&n);
        }
        ContinueSearch::Yes
    });

    println!("{start}");
    println!("{height2locations:?}");
    let mut num_incoming = HashHistogram::new();
    num_incoming.bump(start);
    for height in 0..=8 {
        let height = ModNumC::new(height);
        for p in height2locations.get(&height).unwrap() {
            for next in height2locations.get(&(height + 1)).unwrap() {
                if p.manhattan_distance(next) == 1 {
                    num_incoming.bump_by(next, num_incoming.count(p));
                }
            }
        }
    }
    println!("{num_incoming}");
    for p in height2locations.get(&ModNumC::new(9)).unwrap() {
        println!("{p:?}: {}", num_incoming.count(p));
    }
    height2locations.get(&ModNumC::new(9)).unwrap().iter().map(|p| num_incoming.count(p)).sum()
}
