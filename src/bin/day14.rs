use advent2024::{advent_main, all_lines, multidim::Position};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let robots = all_lines(filename)?.map(Robot::new).collect::<Vec<_>>();
        println!("{robots:?}");
        Ok(())
    })
}

#[derive(Copy, Clone, Debug)]
struct Robot {
    p: Position,
    v: Position,
}

impl Robot {
    fn new(line: String) -> Self {
        let re = regex::Regex::new(r"-?\d+").unwrap();
        let mut iter = re.find_iter(line.as_str()).map(|n| n.as_str().parse::<isize>().unwrap());
        let p = Position::from((iter.next().unwrap(), iter.next().unwrap()));
        let v = Position::from((iter.next().unwrap(), iter.next().unwrap()));
        Self {p, v}
    }
}