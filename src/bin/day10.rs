use advent2024::{advent_main, grid::GridDigitWorld, multidim::{DirType, ManhattanDir}, searchers::{breadth_first_search, ContinueSearch, SearchQueue}, Part};
use bare_metal_modulo::MNum;
use enum_iterator::all;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let topomap = GridDigitWorld::from_digit_file(filename)?;
        let mut total = 0;
        for (p, _) in topomap.position_value_iter().filter(|(_, v)| **v == 0) {
            let mut nine_count = 0;
            let mut branches = 0;
            breadth_first_search(p, |p, q| {
                let height = topomap.value(*p).unwrap();
                if height == 9 {
                    nine_count += 1;
                } else {
                    let enqueued = q.len();
                    for dir in all::<ManhattanDir>() {
                        let n = dir.neighbor(*p);
                        if let Some(up) = topomap.value(n) {
                            if height.a() + 1 == up.a() {
                                q.enqueue(&n);
                            }
                        }
                    }
                    if q.len() - enqueued > 1 {
                        branches += q.len() - enqueued;
                    }
                }
                ContinueSearch::Yes
            });
            println!("{p}: nine_count: {nine_count} branches: {branches}");
            total += match part {
                Part::One => nine_count,
                Part::Two => branches,
            };
        }
        println!("{total}");
        Ok(())
    })
}
