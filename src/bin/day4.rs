use advent2024::{chooser_main, grid::GridCharWorld, multidim::{Dir, DirType}, Part};
use enum_iterator::all;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let world = GridCharWorld::from_char_file(filename)?;
        for dir in all::<Dir>() {
            let (dx, dy) = dir.offset();
            
        }
        Ok(())
    })
}

struct Starts {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    width: isize,
    height: isize,
}

impl Starts {
    fn new(d: Dir, width: isize, height: isize) -> Self {
        let (offx, offy) = d.offset();
        let (x, y, dx, dy);
        if offx > 0 {
            dy = 1;
            y = 0;
        } else if offx < 0 {
            dy = -1;
            y = height - 1;
        } else {
            dy = 0;
            y = 0;
        }
        if offy > 0 {
            dx = 1;
            x = 0;
        } else if offy < 0 {
            dx = -1;
            x = width - 1;
        } else {
            dx = 0;
            x = 0;
        }
        Self {x, y, dx, dy, width, height}
    }

    fn x_in_bounds(&self) -> bool {
        self.x >= 0 && self.x < self.width
    }

    fn y_in_bounds(&self) -> bool {
        self.y >= 0 && self.y < self.height
    }
}

impl Iterator for Starts {
    type Item = (isize, isize);
    
    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.x, self.y);
        if self.dx > 0 {
            self.x += self.dx;
            if !self.x_in_bounds() {
                self.dx = 0;
                self.x = 0;
            }
            Some(result)
        } else if self.dy > 0 {
            self.y += self.dy;
            if !self.y_in_bounds() {
                self.dy = 0;
            }
            Some(result)
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
            let starts = Starts::new(d, 3, 2).collect::<Vec<_>>();
            println!("{d:?}: {starts:?}");
        }
    }
}