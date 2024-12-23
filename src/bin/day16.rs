use std::collections::HashSet;

use advent2024::{
    advent_main,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    search_iter::{BfsIter, PrioritySearchIter},
    Part,
};
use enum_iterator::all;
use itertools::Itertools;

const MOVE_COST: usize = 1;
const TURN_COST: usize = 1000;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let maze = GridCharWorld::from_char_file(filename)?;
        let start = Reindeer::new(maze.any_position_for('S'), ManhattanDir::E);
        let end = maze.any_position_for('E');
        let mut searcher = PrioritySearchIter::dijkstra(start, successor_func(&maze));
        match part {
            Part::One => part1(end, &mut searcher),
            Part::Two => part2(end, &maze, &mut searcher, options.contains(&"-show")),
        }
        Ok(())
    })
}

fn successor_func(maze: &GridCharWorld) -> impl Fn(&Reindeer) -> Vec<(Reindeer, usize)> + '_ {
    |s: &Reindeer| {
        [
            (s.forward(), MOVE_COST),
            (s.left(), TURN_COST),
            (s.right(), TURN_COST),
        ]
        .iter()
        .filter(|(p, _)| maze.value(p.p).map_or(false, |v| v != '#'))
        .copied()
        .collect()
    }
}

fn part1<S: FnMut(&Reindeer) -> Vec<(Reindeer, usize)>, H: Fn(&Reindeer) -> usize>(
    end: Position,
    searcher: &mut PrioritySearchIter<usize, Reindeer, S, H>,
) {
    let at_goal = searcher.find(|r| r.p == end).unwrap();
    let score = searcher.cost_for(&at_goal);
    println!("{score}");
}

fn part2<S: FnMut(&Reindeer) -> Vec<(Reindeer, usize)>, H: Fn(&Reindeer) -> usize>(
    end: Position,
    maze: &GridCharWorld,
    searcher: &mut PrioritySearchIter<usize, Reindeer, S, H>,
    show: bool,
) {
    searcher.by_ref().last();
    let mut on_path = HashSet::new();
    BfsIter::new(end, |p| {
        on_path.insert(*p);
        let outgoing_dirs = all::<ManhattanDir>()
            .filter(|d| on_path.contains(&d.neighbor(*p)))
            .collect::<Vec<_>>();
        let candidates = all::<ManhattanDir>()
            .map(|d| Reindeer::new(d.neighbor(*p), d.inverse()))
            .filter(|r| maze.value(r.p).map_or(false, |v| v != '#'))
            .collect_vec();
        let costs = candidates
            .iter()
            .map(|r| {
                let mut cost = searcher.cost_for(r);
                if !outgoing_dirs.contains(&r.f) {
                    cost += TURN_COST as usize;
                }
                cost
            })
            .collect_vec();
        let min_cost = costs.iter().min().unwrap();
        (0..costs.len())
            .filter(|i| costs[*i] == *min_cost)
            .map(|i| candidates[i].p)
            .collect()
    })
    .last();
    if show {
        let mut maze = maze.clone();
        for p in on_path.iter() {
            maze.update(*p, 'O');
        }
        println!("{maze}");
    }
    println!("{}", on_path.len());
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Reindeer {
    p: Position,
    f: ManhattanDir,
}

impl Reindeer {
    fn new(p: Position, f: ManhattanDir) -> Self {
        Self { p, f }
    }

    fn with_facing(&self, f: ManhattanDir) -> Self {
        Self::new(self.p, f)
    }

    fn with_position(&self, p: Position) -> Self {
        Self::new(p, self.f)
    }

    fn forward(&self) -> Self {
        self.with_position(self.f.neighbor(self.p))
    }

    fn left(&self) -> Self {
        self.with_facing(self.f.counterclockwise())
    }

    fn right(&self) -> Self {
        self.with_facing(self.f.clockwise())
    }
}
