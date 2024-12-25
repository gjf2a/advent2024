use advent2024::{advent_main, all_lines, grid::GridCharWorld, multidim::Position};
use itertools::Itertools;

const PATTERN_WIDTH: usize = 5;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, _part, _| {
        let (keys, locks) = keys_and_locks(filename)?;
        let mut fit = 0;
        for key in keys.iter() {
            for lock in locks.iter() {
                if key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5) {
                    fit += 1;
                }
            }
        }
        println!("{fit}");
        Ok(())
    })
}

fn keys_and_locks(
    filename: &str,
) -> anyhow::Result<(Vec<[usize; PATTERN_WIDTH]>, Vec<[usize; PATTERN_WIDTH]>)> {
    let mut keys = vec![];
    let mut locks = vec![];
    let mut lines = all_lines(filename)?;
    loop {
        let blob = lines.by_ref().take_while(|line| line.len() > 0).join("\n");
        if blob.len() == 0 {
            return Ok((keys, locks));
        } else {
            let pattern = blob.parse::<GridCharWorld>()?;
            let is_lock = (0..PATTERN_WIDTH as isize)
                .map(|x| pattern.value(Position::from((x, 0))).unwrap())
                .all(|v| v == '#');
            let y_dir = if is_lock { 1 } else { -1 };
            (if is_lock { &mut locks } else { &mut keys }).push(heights_from(&pattern, y_dir));
        }
    }
}

fn heights_from(pattern: &GridCharWorld, y_dir: isize) -> [usize; PATTERN_WIDTH] {
    let mut result = [0; PATTERN_WIDTH];
    let y_start = (if y_dir < 0 { pattern.height() - 2 } else { 1 }) as isize;
    for x in 0..result.len() {
        let mut y = y_start;
        while let Some(col_value) = pattern.value(Position::from((x as isize, y))) {
            if col_value == '#' {
                result[x] += 1;
            }
            y += y_dir;
        }
    }
    result
}
