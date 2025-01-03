use bare_metal_modulo::*;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::multidim::{
    map_width_height, to_map, DirType, Position, RingIterator, RowMajorPositionIterator,
};

pub type GridDigitWorld = GridWorld<ModNumC<u8, 10>>;
pub type GridCharWorld = GridWorld<char>;

pub trait CharDisplay {
    fn display(&self) -> char;
}

impl CharDisplay for ModNumC<u8, 10> {
    fn display(&self) -> char {
        (self.a() + '0' as u8) as char
    }
}

impl CharDisplay for char {
    fn display(&self) -> char {
        *self
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GridWorld<V> {
    map: BTreeMap<Position, V>,
    width: usize,
    height: usize,
}

impl GridDigitWorld {
    pub fn from_digit_file(filename: &str) -> anyhow::Result<GridDigitWorld> {
        Self::from_file(filename, |c| ModNumC::new(c.to_digit(10).unwrap() as u8))
    }
}

impl GridCharWorld {
    pub fn from_char_file(filename: &str) -> anyhow::Result<GridCharWorld> {
        Self::from_file(filename, |c| c)
    }
}

fn convert<V: Clone>(map: &HashMap<Position, V>) -> BTreeMap<Position, V> {
    map.iter().map(|(k, v)| (*k, v.clone())).collect()
}

impl FromStr for GridCharWorld {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (col, value) in line.char_indices() {
                map.insert(Position::from((col as isize, row as isize)), value);
            }
        }
        let (width, height) = map_width_height(&map);
        let map = convert(&map);
        Ok(Self { map, width, height })
    }
}

impl<V: Copy + Clone + Eq + PartialEq> GridWorld<V> {
    pub fn from_file<F: Fn(char) -> V>(filename: &str, reader: F) -> anyhow::Result<Self> {
        let map = to_map(filename, reader)?;
        let (width, height) = map_width_height(&map);
        let map = convert(&map);
        Ok(Self { map, width, height })
    }

    pub fn new(width: usize, height: usize, fill_value: V) -> Self {
        let mut map = BTreeMap::new();
        for i in 0..(width as isize) {
            for j in 0..(height as isize) {
                map.insert(Position::from((i, j)), fill_value);
            }
        }
        Self { map, width, height }
    }

    pub fn at_edge(&self, p: Position) -> bool {
        p[0] == 0
            || p[1] == 0
            || p[0] == self.width() as isize - 1
            || p[1] == self.height() as isize - 1
    }

    pub fn in_bounds(&self, p: Position) -> bool {
        self.map.contains_key(&p)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn value(&self, p: Position) -> Option<V> {
        self.map.get(&p).copied()
    }

    pub fn values_from<D: DirType>(&self, p: Position, dir: D, num_values: usize) -> Vec<V> {
        dir.iter_from(p)
            .take(num_values)
            .map(|p| self.value(p))
            .take_while(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect()
    }

    pub fn get(&self, col: usize, row: usize) -> Option<V> {
        self.value(Position::from((col as isize, row as isize)))
    }

    pub fn update(&mut self, p: Position, value: V) {
        if let Some(current) = self.map.get_mut(&p) {
            *current = value;
        }
    }

    pub fn modify<M: FnMut(&mut V)>(&mut self, p: Position, mut modifier: M) {
        self.map.get_mut(&p).map(|v| modifier(v));
    }

    pub fn swap(&mut self, p1: Position, p2: Position) {
        if p1 != p2 && self.map.contains_key(&p1) && self.map.contains_key(&p2) {
            let p1val = self.map.remove(&p1).unwrap();
            let p2val = self.map.remove(&p2).unwrap();
            self.map.insert(p1, p2val);
            self.map.insert(p2, p1val);
        }
    }

    pub fn position_iter(&self) -> RowMajorPositionIterator {
        RowMajorPositionIterator::new(self.width, self.height)
    }

    pub fn position_value_iter(&self) -> impl Iterator<Item = (&Position, &V)> {
        self.map.iter()
    }

    pub fn position_value_iter_mut(&mut self) -> impl Iterator<Item = (&Position, &mut V)> {
        self.map.iter_mut()
    }

    pub fn ring_iter(&self) -> RingIterator {
        RingIterator::new(
            self.map.keys().min().copied().unwrap(),
            self.width as isize,
            self.height as isize,
        )
    }

    pub fn positions_for(&self, item: V) -> BTreeSet<Position> {
        self.position_iter()
            .filter(|p| self.value(*p).unwrap() == item)
            .collect()
    }

    pub fn any_position_for(&self, item: V) -> Position {
        self.positions_for(item).iter().next().copied().unwrap()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl<V: CharDisplay + Copy + Eq + PartialEq> Display for GridWorld<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in self.position_iter() {
            if p[1] > 0 && p[0] == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", self.value(p).unwrap().display())?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct InfiniteGrid<V: Copy + Clone + Debug + Default + Display> {
    map: BTreeMap<Position, V>,
}

impl<V: Copy + Clone + Debug + Default + Display> Display for InfiniteGrid<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ((x_start, y_start), (x_end, y_end)) = self.bounding_box();
        for y in y_start..=y_end {
            for x in x_start..=x_end {
                write!(f, "{}", self.get(x, y))?;
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl<V: Copy + Clone + Debug + Default + Display> InfiniteGrid<V> {
    pub fn get_pos(&self, p: Position) -> V {
        self.map.get(&p).copied().unwrap_or_default()
    }

    pub fn add_pos(&mut self, p: Position, value: V) {
        self.map.insert(p, value);
    }

    pub fn get(&self, x: isize, y: isize) -> V {
        self.get_pos(Position::from((x, y)))
    }

    pub fn add(&mut self, x: isize, y: isize, value: V) {
        self.add_pos(Position::from((x, y)), value)
    }

    pub fn move_square(&mut self, start: (isize, isize), movement: (isize, isize)) {
        let start = Position::from(start);
        let offset = Position::from(movement);
        let value = self.map.remove(&start).unwrap_or_default();
        self.add_pos(start + offset, value);
    }

    pub fn bounding_box(&self) -> ((isize, isize), (isize, isize)) {
        ((self.min_x(), self.min_y()), (self.max_x(), self.max_y()))
    }

    pub fn min_x(&self) -> isize {
        self.map.keys().map(|k| k[0]).min().unwrap()
    }

    pub fn max_x(&self) -> isize {
        self.map.keys().map(|k| k[0]).max().unwrap()
    }

    pub fn min_y(&self) -> isize {
        self.map.keys().map(|k| k[1]).min().unwrap()
    }

    pub fn max_y(&self) -> isize {
        self.map.keys().map(|k| k[1]).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::GridCharWorld;

    #[test]
    fn test_grid_read() {
        let maze_str = ".....##
###.###
#.....#
#.#####
#......";

        let maze = maze_str.parse::<GridCharWorld>().unwrap();
        assert_eq!(maze_str, format!("{maze}").as_str());
    }
}
