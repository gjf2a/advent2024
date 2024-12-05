use std::{cmp::Ordering, collections::BTreeSet};

use advent2024::{all_lines, chooser_main, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let mut pairs = BTreeSet::new();
        let mut count = 0;
        let mut collecting_rules = true;
        for line in all_lines(filename)? {
            if collecting_rules {
                if line.len() > 0 {
                    let mut parts = line.split('|').map(|n| n.parse::<i64>().unwrap());
                    pairs.insert((parts.next().unwrap(), parts.next().unwrap()));
                } else {
                    collecting_rules = false;
                }
            } else {
                let mut update = line.split(",").map(|n| n.parse().unwrap()).collect();
                let correctly_ordered = passes_ordering_rule(&update, &pairs);
                if part == Part::One && correctly_ordered || part == Part::Two && !correctly_ordered
                {
                    if !correctly_ordered {
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
        }
        println!("{count}");

        Ok(())
    })
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
