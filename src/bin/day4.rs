use advent2024::{
    chooser_main,
    grid::GridCharWorld,
    multidim::{Dir, DirType, Position},
    Part,
};
use enum_iterator::all;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let world = GridCharWorld::from_char_file(filename)?;
        let count = match part {
            Part::One => part1(world),
            Part::Two => part2(world),
        };
        println!("{count}");
        Ok(())
    })
}

fn part1(world: GridCharWorld) -> usize {
    let mut count = 0;
    let target = vec!['X', 'M', 'A', 'S'];
    for dir in all::<Dir>() {
        for start in Starts::new(dir, world.width() as isize, world.height() as isize) {
            let mut current = start;
            while world.in_bounds(current) {
                let streak = world.values_from(current, dir, target.len());
                if streak == target {
                    count += 1;
                }
                current = dir.neighbor(current);
            }
        }
    }
    count
}

fn part2(world: GridCharWorld) -> usize {
    let mut count = 0;
    let target = vec!['M', 'A', 'S'];
    let diags = vec![Dir::Nw, Dir::Sw, Dir::Ne, Dir::Se];
    for (p, c) in world.position_value_iter() {
        if *c == 'A' && !world.at_edge(*p) {
            let matching_diags = diags
                .iter()
                .filter(|d| world.values_from(d.neighbor(*p), d.inverse(), target.len()) == target)
                .count();
            if matching_diags == 2 {
                count += 1;
            }
        }
    }
    count
}

#[derive(Debug)]
struct Starts {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    x_restart: isize,
    y_restart: isize,
    width: isize,
    height: isize,
}

impl Starts {
    fn new(d: Dir, width: isize, height: isize) -> Self {
        let (mut x, mut y, dx, dy, x_restart, y_restart) = match d {
            Dir::N => (0, height - 1, 1, 0, 0, 0),
            Dir::Ne => (0, height - 1, 1, -1, 0, height - 2),
            Dir::E => (0, 0, 0, 1, 0, 0),
            Dir::Se => (0, 0, 1, 1, 0, 1),
            Dir::S => (0, 0, 1, 0, 0, 0),
            Dir::Sw => (0, 0, 1, 1, width - 1, 1),
            Dir::W => (0, 0, 0, 1, width - 1, 0),
            Dir::Nw => (0, height - 1, 1, -1, width - 1, height - 2),
        };
        if dx == 0 {
            x = x_restart;
            y = y_restart;
        }
        Self {
            x,
            y,
            dx,
            dy,
            x_restart,
            y_restart,
            width,
            height,
        }
    }

    fn x_in_bounds(&self) -> bool {
        self.x >= 0 && self.x < self.width
    }

    fn y_in_bounds(&self) -> bool {
        self.y >= 0 && self.y < self.height
    }
}

impl Iterator for Starts {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.x, self.y);
        if self.dx != 0 {
            self.x += self.dx;
            if !self.x_in_bounds() {
                self.dx = 0;
                self.x = self.x_restart;
                self.y = self.y_restart;
            }
            Some(Position::from(result))
        } else if self.dy != 0 {
            self.y += self.dy;
            if !self.y_in_bounds() {
                self.dy = 0;
            }
            Some(Position::from(result))
        } else {
            None
        }
    }
}
