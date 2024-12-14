pub mod combinations;
pub mod extended_euclid;
pub mod grid;
pub mod multidim;
pub mod searchers;

use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader, Lines},
    str::FromStr,
    time::Instant,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one" => Ok(Self::One),
            "two" => Ok(Self::Two),
            _ => Err(anyhow::anyhow!("No match for Part")),
        }
    }
}

pub fn advent_main(code: fn(&str, Part, Vec<&str>) -> anyhow::Result<()>) -> anyhow::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} filename [one|two] [options]", args[0]);
    } else if args.len() == 2 {
        code(args[1].as_str(), Part::One, vec![])?;
    } else {
        let options = args
            .iter()
            .skip(3)
            .map(|arg| arg.as_str())
            .collect::<Vec<_>>();
        code(args[1].as_str(), args[2].parse().unwrap(), options)?;
    }
    let duration = Instant::now().duration_since(start);
    println!("duration: {} ms", duration.as_millis());
    Ok(())
}

pub fn all_lines_wrap(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    Ok(io::BufReader::new(fs::File::open(filename)?).lines())
}

pub fn all_lines(filename: &str) -> io::Result<impl Iterator<Item = String>> {
    Ok(all_lines_wrap(filename)?.map(|line| line.unwrap()))
}

#[cfg(test)]
mod tests {
    use enum_iterator::all;

    use crate::{
        multidim::{Dir, DirType, ManhattanDir, Position, RowMajorPositionIterator},
        searchers::{breadth_first_search, AdjacencySets, ContinueSearch, SearchQueue},
    };

    #[test]
    fn test_dir() {
        assert_eq!(
            all::<Dir>().collect::<Vec<Dir>>(),
            vec![
                Dir::N,
                Dir::Ne,
                Dir::E,
                Dir::Se,
                Dir::S,
                Dir::Sw,
                Dir::W,
                Dir::Nw
            ]
        );

        let neighbors = all::<Dir>()
            .map(|d| d.neighbor(Position::from((4, 4))))
            .map(|p| (p[0], p[1]))
            .collect::<Vec<(isize, isize)>>();
        let targets = vec![
            (4, 3),
            (5, 3),
            (5, 4),
            (5, 5),
            (4, 5),
            (3, 5),
            (3, 4),
            (3, 3),
        ];
        assert_eq!(neighbors, targets);

        let mut p = Position::from((3, 2));
        p = Dir::Nw.neighbor(p);
        assert_eq!(p, Position::from((2, 1)));
        p = Dir::Se.neighbor(p);
        assert_eq!(p, Position::from((3, 2)));
        assert_eq!(Dir::Ne.neighbor(p), Position::from((4, 1)));

        let ps: Vec<Position> = RowMajorPositionIterator::new(2, 3).collect();
        let targets = [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)];
        assert_eq!(ps.len(), targets.len());
        assert!((0..targets.len()).all(|i| Position::from(targets[i]) == ps[i]));

        assert_eq!(Dir::N.rotated_degrees(90), Dir::E);
        assert_eq!(Dir::N.rotated_degrees(180), Dir::S);
        assert_eq!(Dir::N.rotated_degrees(270), Dir::W);
        assert_eq!(Dir::N.rotated_degrees(360), Dir::N);
        assert_eq!(Dir::N.rotated_degrees(-90), Dir::W);
        assert_eq!(Dir::E.rotated_degrees(180), Dir::W);
        assert_eq!(Dir::E.rotated_degrees(-180), Dir::W);
    }

    #[test]
    fn test_manhattan() {
        let p = Position::default();
        for (d, (x, y)) in all::<ManhattanDir>().zip([(0, -1), (1, 0), (0, 1), (-1, 0)].iter()) {
            let next = d.neighbor(p);
            assert_eq!(next, Position::from((*x, *y)));
            let inverse = d.inverse().neighbor(next);
            assert_eq!(inverse, p);
        }

        let mut d1 = ManhattanDir::N;
        for d2 in all::<ManhattanDir>() {
            assert_eq!(d1, d2);
            d1 = d1.clockwise();
            assert_eq!(d1.counterclockwise(), d2);
        }
        assert_eq!(d1, ManhattanDir::N);
    }

    #[test]
    fn test_bfs() {
        println!("Test BFS");
        let max_dist = 2;
        let start_value = Position::default();
        println!("Starting BFS");
        let paths_back = breadth_first_search(&start_value, |p, q| {
            for n in p
                .manhattan_neighbors()
                .iter()
                .filter(|n| n.manhattan_distance(&start_value) <= max_dist)
            {
                q.enqueue(&n);
            }
            ContinueSearch::Yes
        });
        println!("Search complete.");
        assert_eq!(paths_back.len(), 13);
        for node in paths_back.keys() {
            let len = paths_back.path_back_from(node).unwrap().len();
            println!("From {:?}: {}", node, len);
            assert!(len <= 1 + max_dist as usize);
        }
    }

    #[test]
    fn graph_test() {
        let mut graph = AdjacencySets::new();
        for (a, b) in [
            ("start", "A"),
            ("start", "b"),
            ("A", "c"),
            ("A", "b"),
            ("b", "d"),
            ("A", "end"),
            ("b", "end"),
        ] {
            graph.connect2(a, b);
        }
        let keys = graph.keys().collect::<Vec<_>>();
        assert_eq!(keys, vec!["A", "b", "c", "d", "end", "start"]);
        let parent_map = breadth_first_search(&"start".to_string(), |node, q| {
            graph
                .neighbors_of(node)
                .unwrap()
                .iter()
                .for_each(|n| q.enqueue(n));
            ContinueSearch::Yes
        });
        let parent_map_str = format!("{:?}", parent_map);
        assert_eq!(
            parent_map_str.as_str(),
            r#"ParentMap { parents: {"start": None, "A": Some("start"), "b": Some("start"), "c": Some("A"), "end": Some("A"), "d": Some("b")}, last_dequeued: Some("d") }"#
        );
        let path = parent_map.path_back_from(&"end".to_string()).unwrap();
        let path_str = format!("{:?}", path);
        assert_eq!(path_str, r#"["start", "A", "end"]"#);
    }
}
