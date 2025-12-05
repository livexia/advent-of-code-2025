use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type IdRange = (usize, usize);

fn parse_input<T: AsRef<str>>(input: T) -> Result<(Vec<IdRange>, Vec<usize>)> {
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

fn part1(ranges: &[IdRange], ids: &[usize]) -> Result<usize> {
    let _start = Instant::now();

    let count = ids
        .iter()
        .filter(|&&id| ranges.iter().any(|r| r.0 <= id && r.1 >= id))
        .count();

    println!("part 1: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn merge_range(r: IdRange, other: IdRange) -> Option<IdRange> {
    let (r, other) = if r.0 > other.0 {
        (other, r)
    } else {
        (r, other)
    };
    if r.1 < other.0 {
        None
    } else if r.1 >= other.1 {
        Some(r)
    } else {
        Some((r.0, other.1))
    }
}

fn part2(ranges: &[IdRange]) -> Result<usize> {
    let _start = Instant::now();

    let mut ranges = ranges.to_vec();
    let mut merged = vec![];

    let mut flag = true;

    while flag {
        flag = false;
        merged.clear();
        'outer: while let Some(other) = ranges.pop() {
            for r in merged.iter_mut() {
                if let Some(m) = merge_range(*r, other) {
                    flag = true;
                    *r = m;
                    continue 'outer;
                } else {
                    continue;
                }
            }
            merged.push(other);
        }
        ranges = merged.clone();
    }

    let count = merged.iter().map(|(s, e)| e - s + 1).sum();

    println!("part 2: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (ranges, ids) = parse_input(input)?;

    part1(&ranges, &ids)?;
    part2(&ranges)?;
    Ok(())
}

#[test]
fn test_merge() -> Result<()> {
    assert_eq!(merge_range((10, 15), (18, 22)), None);
    assert_eq!(merge_range((10, 19), (18, 22)), Some((10, 22)));
    assert_eq!(merge_range((10, 19), (12, 18)), Some((10, 19)));
    assert_eq!(merge_range((10, 19), (1, 15)), Some((1, 19)));
    assert_eq!(merge_range((10, 19), (1, 8)), None);
    assert_eq!(merge_range((10, 20), (12, 18)), Some((10, 20)));
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
    assert_eq!(part2(&ranges).unwrap(), 14);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (ranges, ids) = parse_input(input)?;
    assert_eq!(part1(&ranges, &ids).unwrap(), 623);
    assert_eq!(part2(&ranges).unwrap(), 353507173555373);
    Ok(())
}
