use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (isize, isize);

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Coord>> {
    input
        .as_ref()
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            if let Some((x, y)) = l.split_once(",") {
                Ok((x.parse().unwrap(), y.parse().unwrap()))
            } else {
                err!("unable to parse: {l:?}")
            }
        })
        .collect()
}

fn area(c: Coord, other: Coord) -> usize {
    (1 + c.0.abs_diff(other.0)) * (c.1.abs_diff(other.1) + 1)
}

fn part1(grid: &[Coord]) -> Result<usize> {
    let _start = Instant::now();

    let mut largest = 0;
    for i in 0..grid.len() {
        for j in i + 1..grid.len() {
            largest = largest.max(area(grid[i], grid[j]))
        }
    }

    println!("part 1: {largest}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(largest)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let grid = parse_input(input)?;
    part1(&grid)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    let grid = parse_input(input)?;
    assert_eq!(part1(&grid).unwrap(), 50);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let grid = parse_input(input)?;
    assert_eq!(part1(&grid).unwrap(), 4765757080);
    Ok(())
}
