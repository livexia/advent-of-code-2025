use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<(Vec<(usize, usize)>, Vec<usize>)> {
    if let Some((ranges, ids)) = input.as_ref().trim().split_once("\n\n") {
        let ranges = ranges
            .trim()
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                if let Some((s, e)) = l.trim().split_once("-") {
                    Ok((s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()))
                } else {
                    err!("unable to parse line: {l}")
                }
            })
            .collect::<Result<_>>()?;
        let ids = ids
            .trim()
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.trim()
                    .parse::<usize>()
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
            })
            .collect::<Result<_>>()?;
        Ok((ranges, ids))
    } else {
        err!("unable to parse input")
    }
}

fn part1(ranges: &[(usize, usize)], ids: &[usize]) -> Result<usize> {
    let _start = Instant::now();

    let count = ids
        .iter()
        .filter(|&&id| ranges.iter().any(|r| r.0 <= id && r.1 >= id))
        .count();

    println!("part 1: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (ranges, ids) = parse_input(input)?;

    part1(&ranges, &ids)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    let (ranges, ids) = parse_input(input)?;
    assert_eq!(part1(&ranges, &ids).unwrap(), 3);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (ranges, ids) = parse_input(input)?;
    assert_eq!(part1(&ranges, &ids).unwrap(), 623);
    assert_eq!(2, 2);
    Ok(())
}
