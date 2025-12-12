use std::error::Error;
use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[allow(dead_code)]
#[derive(Debug)]
struct Present {
    _index: usize,
    shapes: Vec<(isize, isize)>,
}

#[derive(Debug)]
struct Region {
    size: (usize, usize),
    presents: Vec<usize>,
}

impl FromStr for Present {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        let lines: Vec<_> = s
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.trim())
            .collect();
        if let Some(id) = lines[0].strip_suffix(":") {
            let id: usize = id.trim().parse()?;
            let mut shapes = vec![];
            for (i, line) in lines[..].iter().enumerate() {
                for (j, c) in line.chars().enumerate() {
                    if c == '#' {
                        shapes.push((i as isize, j as isize));
                    }
                }
            }
            return Ok(Self { _index: id, shapes });
        }
        err!("unable to parse present: {s:?}")
    }
}

impl FromStr for Region {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((size, numbers)) = s.split_once(":")
            && let Some((x, y)) = size.trim().split_once("x")
        {
            let size: (usize, usize) = (x.trim().parse()?, y.trim().parse()?);
            let numbers: Vec<usize> = numbers
                .split_whitespace()
                .map(|n| n.parse::<usize>().map_err(|e| e.into()))
                .collect::<Result<_>>()?;
            return Ok(Self {
                size,
                presents: numbers,
            });
        }
        err!("unable to parse region: {s:?}")
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Result<(Vec<Present>, Vec<Region>)> {
    let mut presents = vec![];
    let mut regions = vec![];
    for part in input
        .as_ref()
        .trim()
        .split("\n\n")
        .filter(|p| !p.trim().is_empty())
    {
        match part.parse::<Present>() {
            Ok(present) => presents.push(present),
            Err(present_e) => {
                for line in part.trim().lines() {
                    match line.parse::<Region>() {
                        Ok(region) => regions.push(region),
                        Err(region_e) => return err!("{present_e}\n{region_e}"),
                    }
                }
            }
        }
    }
    Ok((presents, regions))
}

impl Present {}

impl Region {
    fn try_fit(&self, _presents: &[Present]) -> bool {
        let (x, y) = self.size;
        // present is all 3x3
        let count: usize = self.presents.iter().sum();
        count <= (x / 3) * (y / 3)
    }
}

fn part1(presents: &[Present], regions: &[Region]) -> Result<usize> {
    let _start = Instant::now();

    let count = regions.iter().filter(|r| r.try_fit(presents)).count();

    println!("part 1: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (presents, regions) = parse_input(input)?;

    part1(&presents, &regions)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
    let (presents, regions) = parse_input(input)?;
    assert_eq!(part1(&presents, &regions).unwrap(), 2);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (presents, regions) = parse_input(input)?;
    assert_eq!(part1(&presents, &regions).unwrap(), 531);
    Ok(())
}
