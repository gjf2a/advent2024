use std::collections::BTreeMap;

use advent2024::{
    advent_main,
    grid::GridDigitWorld,
    multidim::{DirType, ManhattanDir, Position},
    search_iter::BfsIter,
    searchers::{breadth_first_search, ContinueSearch, SearchQueue},
    Part,
};
use bare_metal_modulo::{MNum, ModNumC};
use enum_iterator::all;
use hash_histogram::HashHistogram;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let topomap = GridDigitWorld::from_digit_file(filename)?;
        if options.contains(&"-dynamic") {
            dynamic_versions(part, &topomap);
        } else {
            original_versions(part, &topomap);
        }
        Ok(())
    })
}

fn dynamic_versions(part: Part, topomap: &GridDigitWorld) {
    let result = match part {
        Part::One => {
            let mut total = 0;
            for (start, _) in topomap.position_value_iter().filter(|(_, h)| **h == 0) {
                total += pure_dynamic(|s, _| s == start, topomap)
                    .iter()
                    .filter(|(_, c)| *c > 0)
                    .count();
            }
            total
        }
        Part::Two => pure_dynamic(|_, h| h == 0, topomap)
            .iter()
            .map(|(_, c)| c)
            .sum::<usize>(),
    };
    println!("{result}");
}

fn pure_dynamic<S: Fn(&Position, u8) -> bool>(
    start_predicate: S,
    topomap: &GridDigitWorld,
) -> Vec<(Position, usize)> {
    let (height2locations, mut paths_to) = dynamic_tables(start_predicate, topomap);

    for (height, locations) in height2locations.iter().filter(|(h, _)| **h < 9) {
        for p in locations.iter() {
            for next in height2locations.get(&(height + 1)).unwrap() {
                if p.manhattan_distance(next) == 1 {
                    paths_to.bump_by(next, paths_to.count(p));
                }
            }
        }
    }

    let nines = height2locations.get(&9).unwrap();
    nines.iter().map(|p| (*p, paths_to.count(&p))).collect()
}

fn dynamic_tables<S: Fn(&Position, u8) -> bool>(
    start_predicate: S,
    topomap: &GridDigitWorld,
) -> (BTreeMap<u8, Vec<Position>>, HashHistogram<Position>) {
    let mut height2locations = BTreeMap::new();
    let mut paths_to = HashHistogram::new();
    for (p, h) in topomap.position_value_iter() {
        let height = h.a();
        if start_predicate(p, height) {
            paths_to.bump(p);
        }
        match height2locations.get_mut(&height) {
            None => {
                height2locations.insert(height, vec![*p]);
            }
            Some(v) => v.push(*p),
        };
    }
    (height2locations, paths_to)
}

fn original_versions(part: Part, topomap: &GridDigitWorld) {
    let mut total = 0;
    for (start, _) in topomap.position_value_iter().filter(|(_, v)| **v == 0) {
        total += match part {
            Part::One => num_reachable_peaks(start, topomap),
            Part::Two => num_distinct_paths(start, topomap),
        }
    }
    println!("{total}");
}

fn num_reachable_peaks(start: &Position, topomap: &GridDigitWorld) -> usize {
    BfsIter::new(*start, |p| ascending_neighbors(*p, topomap).collect())
        .filter(|p| topomap.value(*p).unwrap() == 9)
        .count()
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

fn height2locations(
    start: &Position,
    topomap: &GridDigitWorld,
) -> BTreeMap<ModNumC<u8, 10>, Vec<Position>> {
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

fn num_paths_to(
    start: &Position,
    height2locations: &BTreeMap<ModNumC<u8, 10>, Vec<Position>>,
) -> HashHistogram<Position> {
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
