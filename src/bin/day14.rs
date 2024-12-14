use advent2024::{advent_main, all_lines, multidim::Position};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let mut robots = all_lines(filename)?.map(Robot::new).collect::<Vec<_>>();
        let (width, height) = if filename.contains("ex") {(11, 7)} else {(103, 101)};
        let dimensions = Position::from((width, height));
        for _ in 0..100 {
            for r in robots.iter_mut() {
                r.advance(dimensions);
            }
        }
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
        let mut iter = re
            .find_iter(line.as_str())
            .map(|n| n.as_str().parse::<isize>().unwrap());
        let p = Position::from((iter.next().unwrap(), iter.next().unwrap()));
        let v = Position::from((iter.next().unwrap(), iter.next().unwrap()));
        Self { p, v }
    }

    fn advance(&mut self, dimensions: Position) {
        self.p = (self.p + self.v) % dimensions;
    }

    fn dir_from_center(&self, dimensions: Position) -> Dir {
        
    }
}
