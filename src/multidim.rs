use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
    iter::Sum,
    mem,
    ops::{Add, Index, IndexMut, Mul, Neg, Sub},
    str::FromStr,
};

use bare_metal_modulo::NumType;
use enum_iterator::Sequence;

use crate::all_lines;

pub type Position = Point<isize, 2>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point<N: NumType + Default, const S: usize> {
    coords: [N; S],
}

impl Position {
    pub fn from(pair: (isize, isize)) -> Self {
        Self::new([pair.0, pair.1])
    }

    pub fn next_in_grid(&self, width: usize, height: usize) -> Option<Position> {
        let mut result = self.clone();
        result[0] += 1;
        if result[0] == width as isize {
            result[0] = 0;
            result[1] += 1;
        }
        if result[1] < height as isize {
            Some(result)
        } else {
            None
        }
    }
}

impl<N: NumType + Default, const S: usize> Point<N, S> {
    pub fn new(coords: [N; S]) -> Self {
        Self { coords }
    }

    pub fn values(&self) -> impl Iterator<Item = N> + '_ {
        self.coords.iter().copied()
    }

    pub fn from_iter<I: Iterator<Item = N>>(items: I) -> Self {
        let mut result = Self::default();
        for (i, item) in items.enumerate() {
            result[i] = item;
        }
        result
    }

    pub fn min_max_points<I: Iterator<Item = Point<N, S>>>(
        mut points: I,
    ) -> Option<(Point<N, S>, Point<N, S>)> {
        if let Some(init) = points.next() {
            let init = (init, init);
            Some(points.fold(init, |a, b| {
                (
                    Self::from_iter((0..S).map(|i| min(a.0[i], b[i]))),
                    Self::from_iter((0..S).map(|i| max(a.1[i], b[i]))),
                )
            }))
        } else {
            None
        }
    }

    pub fn bounding_box<I: Iterator<Item = Point<N, S>>>(points: I) -> Option<Vec<Point<N, S>>> {
        Self::min_max_points(points).map(|(ul, lr)| {
            let mut result = vec![];
            Self::bb_help(&mut result, [N::default(); S], &ul, &lr, 0);
            result
        })
    }

    fn bb_help(
        result: &mut Vec<Point<N, S>>,
        coords: [N; S],
        a: &Point<N, S>,
        b: &Point<N, S>,
        start: usize,
    ) {
        if start == coords.len() {
            result.push(Point::new(coords));
        } else {
            for use_a in [true, false] {
                let mut copied = coords;
                copied[start] = (if use_a { a } else { b })[start];
                Self::bb_help(result, copied, a, b, start + 1);
            }
        }
    }
}

impl<N: NumType + num::traits::Signed + Sum<N> + Default, const S: usize> Point<N, S> {
    pub fn manhattan_distance(&self, other: &Point<N, S>) -> N {
        (0..S).map(|i| (self[i] - other[i]).abs()).sum()
    }

    pub fn manhattan_neighbors(&self) -> Vec<Point<N, S>> {
        let mut result = vec![];
        for sign in [N::one(), -N::one()] {
            for pos in 0..S {
                let mut n = self.clone();
                n[pos] += sign;
                result.push(n);
            }
        }
        result
    }

    pub fn adjacent(&self, other: &Point<N, S>) -> bool {
        self.manhattan_distance(other) == N::one()
    }
}

impl<N: NumType + Default, const S: usize> Index<usize> for Point<N, S> {
    type Output = N;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index]
    }
}

impl<N: NumType + Default, const S: usize> IndexMut<usize> for Point<N, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coords[index]
    }
}

impl<N: NumType + Default, const S: usize> Default for Point<N, S> {
    fn default() -> Self {
        Self {
            coords: [N::default(); S],
        }
    }
}

impl<N: NumType + Default, const S: usize> Add for Point<N, S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..S {
            result[i] = self[i] + rhs[i];
        }
        result
    }
}

impl<N: NumType + Default + Neg<Output = N>, const S: usize> Neg for Point<N, S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..S {
            result[i] = -self[i];
        }
        result
    }
}

impl<N: NumType + Default + Neg<Output = N>, const S: usize> Sub for Point<N, S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<N: NumType + Default, const S: usize> Mul<N> for Point<N, S> {
    type Output = Point<N, S>;

    fn mul(self, rhs: N) -> Self::Output {
        let mut result = self;
        for i in 0..S {
            result[i] *= rhs;
        }
        result
    }
}

impl<N: NumType + Default, const S: usize> Display for Point<N, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, c) in self.coords.iter().enumerate() {
            write!(f, "{}", c)?;
            if i < self.coords.len() - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, ")")
    }
}

impl<N: NumType + Default + FromStr, const S: usize> FromStr for Point<N, S>
where
    <N as FromStr>::Err: 'static + Sync + Send + std::error::Error,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = [N::default(); S];
        let mut s = s;
        if s.starts_with('(') && s.ends_with(')') {
            s = &s[1..s.len() - 1];
        }
        for (i, coord) in s.split(",").enumerate() {
            coords[i] = coord.trim().parse()?;
        }
        Ok(Self { coords })
    }
}

pub trait DirType: Copy {
    fn offset(&self) -> Position;

    fn clockwise(&self) -> Self;

    fn counterclockwise(&self) -> Self;

    fn inverse(&self) -> Self;

    fn iter_from(&self, p: Position) -> DirIter<Self> {
        DirIter { p, d: *self }
    }

    fn clockwises(&self, c: usize) -> Self {
        let mut result = *self;
        for _ in 0..c {
            result = result.clockwise();
        }
        result
    }

    fn neighbor(&self, p: Position) -> Position {
        p + self.offset()
    }
}

#[derive(Copy, Clone)]
pub struct DirIter<D: DirType> {
    p: Position,
    d: D,
}

impl<D: DirType> Iterator for DirIter<D> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.p;
        self.p = self.d.neighbor(p);
        Some(p)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Sequence, Hash)]
pub enum ManhattanDir {
    N,
    E,
    S,
    W,
}

impl DirType for ManhattanDir {
    fn offset(&self) -> Position {
        Position::from(match self {
            ManhattanDir::N => (0, -1),
            ManhattanDir::E => (1, 0),
            ManhattanDir::S => (0, 1),
            ManhattanDir::W => (-1, 0),
        })
    }

    fn clockwise(&self) -> ManhattanDir {
        match self {
            ManhattanDir::N => ManhattanDir::E,
            ManhattanDir::E => ManhattanDir::S,
            ManhattanDir::S => ManhattanDir::W,
            ManhattanDir::W => ManhattanDir::N,
        }
    }

    fn counterclockwise(&self) -> ManhattanDir {
        match self {
            ManhattanDir::N => ManhattanDir::W,
            ManhattanDir::W => ManhattanDir::S,
            ManhattanDir::S => ManhattanDir::E,
            ManhattanDir::E => ManhattanDir::N,
        }
    }

    fn inverse(&self) -> ManhattanDir {
        match self {
            ManhattanDir::N => ManhattanDir::S,
            ManhattanDir::S => ManhattanDir::N,
            ManhattanDir::E => ManhattanDir::W,
            ManhattanDir::W => ManhattanDir::E,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Sequence, Hash)]
pub enum Dir {
    N,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Nw,
}

impl DirType for Dir {
    fn offset(&self) -> Position {
        Position::from(match self {
            Dir::N => (0, -1),
            Dir::Ne => (1, -1),
            Dir::E => (1, 0),
            Dir::Se => (1, 1),
            Dir::S => (0, 1),
            Dir::Sw => (-1, 1),
            Dir::W => (-1, 0),
            Dir::Nw => (-1, -1),
        })
    }

    fn clockwise(&self) -> Dir {
        match self {
            Dir::N => Dir::Ne,
            Dir::Ne => Dir::E,
            Dir::E => Dir::Se,
            Dir::Se => Dir::S,
            Dir::S => Dir::Sw,
            Dir::Sw => Dir::W,
            Dir::W => Dir::Nw,
            Dir::Nw => Dir::N,
        }
    }

    fn counterclockwise(&self) -> Dir {
        match self {
            Dir::N => Dir::Nw,
            Dir::Nw => Dir::W,
            Dir::W => Dir::Sw,
            Dir::Sw => Dir::S,
            Dir::S => Dir::Se,
            Dir::Se => Dir::E,
            Dir::E => Dir::Ne,
            Dir::Ne => Dir::N,
        }
    }

    fn inverse(&self) -> Dir {
        self.clockwises(4)
    }
}

impl Dir {
    pub fn rotated_degrees(&self, degrees: isize) -> Dir {
        let mut steps = normalize_degrees(degrees) / 45;
        let mut result = *self;
        while steps > 0 {
            steps -= 1;
            result = result.clockwise();
        }
        result
    }
}

pub fn normalize_degrees(degrees: isize) -> isize {
    let mut degrees = degrees;
    while degrees < 0 {
        degrees += 360;
    }
    degrees % 360
}

pub struct RowMajorPositionIterator {
    width: usize,
    height: usize,
    next: Option<Position>,
}

impl RowMajorPositionIterator {
    pub fn new(width: usize, height: usize) -> Self {
        RowMajorPositionIterator {
            width,
            height,
            next: Some(Position::from((0, 0))),
        }
    }

    pub fn in_bounds(&self) -> bool {
        self.next.map_or(false, |n| {
            n[0] < self.width as isize && n[1] < self.height as isize
        })
    }
}

impl Iterator for RowMajorPositionIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let mut future = self
            .next
            .and_then(|p| p.next_in_grid(self.width, self.height));
        mem::swap(&mut future, &mut self.next);
        future
    }
}

pub struct RingIterator {
    current: Position,
    start: Position,
    end: Position,
    direction: ManhattanDir,
    done: bool,
}

impl RingIterator {
    pub fn new(start: Position, width: isize, height: isize) -> Self {
        Self {
            current: start,
            start: start,
            end: Position::from((start[0] + width - 1, start[1] + height - 1)),
            direction: ManhattanDir::E,
            done: false,
        }
    }

    fn in_bounds(&self, p: Position) -> bool {
        self.start[1] <= p[1] && p[1] <= self.end[1] && self.start[0] <= p[0] && p[0] <= self.end[0]
    }
}

impl Iterator for RingIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let result = self.current;
            let mut candidate = self.direction.neighbor(self.current);
            if !self.in_bounds(candidate) {
                self.direction = self.direction.clockwise();
                candidate = self.direction.neighbor(self.current);
            }
            self.done = candidate == self.start;
            self.current = candidate;
            Some(result)
        }
    }
}

pub fn to_map<V, F: Fn(char) -> V>(
    filename: &str,
    reader: F,
) -> anyhow::Result<HashMap<Position, V>> {
    let mut result = HashMap::new();
    for (row, line) in all_lines(filename)?.enumerate() {
        for (col, value) in line.chars().enumerate() {
            result.insert(Position::from((col as isize, row as isize)), reader(value));
        }
    }
    Ok(result)
}

pub fn map_width_height<V>(map: &HashMap<Position, V>) -> (usize, usize) {
    let max = map.keys().max().unwrap();
    let min = map.keys().min().unwrap();
    (
        (max[0] - min[0] + 1) as usize,
        (max[1] - min[1] + 1) as usize,
    )
}
