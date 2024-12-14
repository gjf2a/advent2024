use advent2024::{advent_main, all_lines, multidim::{Dir, ManhattanDir, Position}};
use hash_histogram::HashHistogram;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, _part, _| {
        let mut robots = all_lines(filename)?.map(Robot::new).collect::<Vec<_>>();
        let (width, height) = if filename.contains("ex") {(11, 7)} else {(103, 101)};
        let dimensions = Position::from((width, height));
        for _ in 0..100 {
            for r in robots.iter_mut() {
                r.advance(dimensions);
            }
        }
        let counts = robots.iter().filter_map(|r| r.dir_from_center(dimensions)).filter(|d| ManhattanDir::try_from(*d).is_err()).collect::<HashHistogram<Dir>>();
        let total = counts.counts().product::<usize>();
        println!("{total}");
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

    fn dir_from_center(&self, dimensions: Position) -> Option<Dir> {
        let center = dimensions / 2;
        let offset = self.p - center;
        if offset[0] == 0 {
            if offset[1] == 0 {
                None
            } else if offset[1] < 0 {
                Some(Dir::N)
            } else {
                Some(Dir::S)
            }
        } else if offset[1] == 0 {
            if offset[0] < 0 {
                Some(Dir::W)
            } else {
                Some(Dir::E)
            }
        } else if offset[0] < 0 {
            if offset[1] < 0 {
                Some(Dir::Nw)
            } else {
                Some(Dir::Sw)
            }
        } else {
            if offset[1] < 0 {
                Some(Dir::Ne)
            } else {
                Some(Dir::Se)
            }
        }
    }
}
