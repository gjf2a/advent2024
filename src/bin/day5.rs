use std::{cmp::Ordering, collections::BTreeSet};

use advent2024::{all_lines, chooser_main, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let mut lines = all_lines(filename)?;
        let pairs = collect_rules_from(&mut lines);
        println!("{}", add_up_medians(part, &pairs, lines));
        Ok(())
    })
}

fn collect_rules_from(lines: &mut impl Iterator<Item = String>) -> BTreeSet<(i64, i64)> {
    lines
        .by_ref()
        .take_while(|s| s.len() > 0)
        .map(|line| {
            let mut parts = line.split('|').map(|n| n.parse().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

fn add_up_medians(
    part: Part,
    pairs: &BTreeSet<(i64, i64)>,
    lines: impl Iterator<Item = String>,
) -> i64 {
    let mut count = 0;
    for line in lines {
        let mut update = line.split(",").map(|n| n.parse().unwrap()).collect();
        if (part == Part::One) == passes_ordering_rule(&update, pairs) {
            if part == Part::Two {
                update.sort_unstable_by(|a, b| {
                    if pairs.contains(&(*a, *b)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
            }
            count += update[update.len() / 2];
        }
    }
    count
}

fn passes_ordering_rule(update: &Vec<i64>, pairs: &BTreeSet<(i64, i64)>) -> bool {
    for i in 0..update.len() {
        for j in (i + 1)..update.len() {
            if pairs.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }
    true
}
