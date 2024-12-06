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

pub fn chooser_main(code: fn(&str, Part, &[String]) -> anyhow::Result<()>) -> anyhow::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} filename [one|two] [options]", args[0]);
    } else if args.len() == 2 {
        code(args[1].as_str(), Part::One, &[])?;
    } else {
        code(args[1].as_str(), args[2].parse().unwrap(), &args[3..])?;
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
