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
    println!("{start}");
    let mut nine_count = 0;
    breadth_first_search(start, |p, q| {
        let height = topomap.value(*p).unwrap();
        if height == 9 {
            nine_count += 1;
            println!("nine at: {p}");
        } else {
            for n in ascending_neighbors(*p, topomap) {
                q.enqueue(&n);
            }
        }
        ContinueSearch::Yes
    });
    let height2locations = height2locations(start, topomap);
    let paths_to = num_paths_to(start, &height2locations);
    let nines = height2locations.get(&ModNumC::new(9)).unwrap();
    let alt1 = nines.len();
    let alt2 = nines.iter().filter(|p| paths_to.count(p) > 0).count();
    println!("nine_count: {nine_count} alt1: {alt1}? {}, alt2: {alt2}? {}", nine_count == alt1, nine_count == alt2);
    nine_count
}

fn ascending_neighbors(
    p: Position,
    topomap: &GridDigitWorld,
) -> impl Iterator<Item = Position> + '_ {
    let target_height = topomap.value(p).unwrap().a() + 1;
    all::<ManhattanDir>()
        .map(move |d| d.neighbor(p))
        .filter(move |n| topomap.value(*n).map_or(false, |h| h.a() == target_height))
}

fn num_distinct_paths(start: &Position, topomap: &GridDigitWorld) -> usize {
    let height2locations = height2locations(start, topomap);
    let paths_to = num_paths_to(start, &height2locations);
    let nines = height2locations.get(&ModNumC::new(9)).unwrap();
    nines.iter().map(|p| paths_to.count(p)).sum()
}

fn height2locations(start: &Position, topomap: &GridDigitWorld) -> BTreeMap<ModNumC<u8, 10>, Vec<Position>> {
    let mut height2locations = BTreeMap::new();
    breadth_first_search(start, |p, q| {
        let height = topomap.value(*p).unwrap();
        match height2locations.get_mut(&height) {
            None => {
                height2locations.insert(height, vec![*p]);
            }
            Some(v) => v.push(*p),
        };
        for n in ascending_neighbors(*p, topomap) {
            q.enqueue(&n);
        }
        ContinueSearch::Yes
    });
    height2locations
}

fn num_paths_to(start: &Position, height2locations: &BTreeMap<ModNumC<u8, 10>, Vec<Position>>) -> HashHistogram<Position> {
    let mut paths_to = HashHistogram::new();
    paths_to.bump(start);
    for height in 0..=8 {
        let height = ModNumC::new(height);
        for p in height2locations.get(&height).unwrap() {
            for next in height2locations.get(&(height + 1)).unwrap() {
                if p.manhattan_distance(next) == 1 {
                    paths_to.bump_by(next, paths_to.count(p));
                }
            }
        }
    }
    paths_to
}