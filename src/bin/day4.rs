use advent2024::{chooser_main, grid::GridCharWorld, multidim::{Dir, DirType, Position}, Part};
use enum_iterator::all;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let target = vec!['X', 'M', 'A', 'S'];
        let world = GridCharWorld::from_char_file(filename)?;
        let mut world_track = GridCharWorld::new(world.width(), world.height(), '.');

        let mut count = 0;
        for dir in all::<Dir>() {
            for start in Starts::new(dir, world.width() as isize, world.height() as isize) {
                let mut current = start;
                while world.in_bounds(current) {
                    let streak = world.values_from(current, dir, target.len());
                    if streak == target {count += 1; world_track.update(current, 'X'); println!("{count}: {current}");}
                    current = dir.neighbor(current);
                }
            }
        }
        println!("{count}");
        println!("{world_track}");
        Ok(())
    })
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
        Self {x, y, dx, dy, x_restart, y_restart, width, height}
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

#[cfg(test)]
mod tests {
    use advent2024::multidim::Dir;
    use enum_iterator::all;

    use crate::Starts;

    #[test]
    fn test_starts() {
        for d in all::<Dir>() {
            let starts = Starts::new(d, 4, 3).collect::<Vec<_>>();
            println!("{d:?}: {starts:?}");
        }
    }
}