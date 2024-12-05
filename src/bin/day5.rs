use std::collections::BTreeSet;

use advent2024::{all_lines, chooser_main};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let mut pairs = BTreeSet::new();
        let mut count = 0;
        let mut phase1 = true;
        for line in all_lines(filename)? {
            if phase1 {
                if line.len() > 0 {
                    let mut parts = line.split('|').map(|n| n.parse::<i64>().unwrap());
                    pairs.insert((parts.next().unwrap(), parts.next().unwrap()));
                } else {
                    phase1 = false;
                    println!("{pairs:?}")
                }
            } else {
                let update = line.split(",").map(|n| n.parse().unwrap()).collect();
                println!("{update:?}");
                if passes_ordering_rule(&update, &pairs) {
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
        for j in (i+1)..update.len() {
            if pairs.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }
    true
}