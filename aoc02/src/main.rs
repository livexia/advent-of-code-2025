use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<(usize, usize)>> {
    input
        .as_ref()
        .split(",")
        .map(|rg| {
            if let Some((start, end)) = rg.trim().split_once("-") {
                Ok((start.trim().parse().unwrap(), end.trim().parse().unwrap()))
            } else {
                err!("Unable to parse input ranges: {rg:?}")
            }
        })
        .collect()
}

fn part1(ranges: &[(usize, usize)]) -> Result<usize> {
    let _start = Instant::now();

    let mut ids = 0;

    for &(start, end) in ranges {
        for n in start..=end {
            let l = n.ilog10() + 1;
            if l % 2 == 0 {
                let base = 10usize.pow(l / 2);
                let right = n % base;
                let left = n / base;
                if left == right {
                    ids += n;
                }
            }
        }
    }

    println!("part1: {ids}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(ids)
}

fn part2(ranges: &[(usize, usize)]) -> Result<usize> {
    let _start = Instant::now();

    let mut ids = 0;

    for &(start, end) in ranges {
        for n in start..=end {
            let l = n.ilog10() + 1;
            'check_base: for base in 1..=l / 2 {
                if l % base == 0 {
                    let base = 10usize.pow(base);

                    let right = n % base;
                    let mut temp = n / base;
                    while temp != 0 {
                        if temp % base != right {
                            continue 'check_base;
                        }
                        temp /= base;
                    }
                    ids += n;
                    break;
                }
            }
        }
    }

    println!("part2: {ids}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(ids)
}

fn split_range(start: usize, end: usize) -> Vec<(usize, usize)> {
    let (start_l, end_l) = (start.ilog10(), end.ilog10());
    if start_l < end_l {
        let mut ranges = vec![];
        let mut start = start;
        for i in start_l..=end_l {
            let new_end = 10usize.pow(i + 1) - 1;
            ranges.push((start, new_end.min(end)));
            start = new_end + 1;
        }
        ranges
    } else {
        vec![(start, end)]
    }
}

fn find_invalid(start: usize, end: usize, base: u32) -> Vec<usize> {
    assert_eq!(start.ilog10(), end.ilog10());
    let l = start.ilog10() + 1;
    if !l.is_multiple_of(base) {
        return vec![];
    }
    let (start_left, end_left) = (start / 10usize.pow(l - base), end / 10usize.pow(l - base));
    let mut invalids = Vec::new();
    for s in start_left..=end_left {
        let n = (0..l)
            .step_by(base as usize)
            .fold(0, |n, i| n + s * 10usize.pow(i));
        if start <= n && n <= end {
            invalids.push(n);
        }
    }
    invalids
}

fn part1_step(ranges: &[(usize, usize)]) -> Result<usize> {
    let _start = Instant::now();

    let mut ids = 0;

    for &(start, end) in ranges {
        for (start, end) in split_range(start, end) {
            let l = start.ilog10() + 1;
            if l % 2 == 0 {
                ids += find_invalid(start, end, l / 2).iter().sum::<usize>()
            }
        }
    }

    println!("part1 by step: {ids}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(ids)
}

fn part2_step(ranges: &[(usize, usize)]) -> Result<usize> {
    let _start = Instant::now();

    let mut invalids = HashSet::new();

    for &(start, end) in ranges {
        for (start, end) in split_range(start, end) {
            let l = start.ilog10() + 1;
            for base in 1..=l / 2 {
                if l % base == 0 {
                    invalids.extend(find_invalid(start, end, base).iter());
                }
            }
        }
    }
    let ids = invalids.iter().sum::<usize>();

    println!("part2 by step: {ids}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(ids)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let ranges = parse_input(input)?;

    part1(&ranges)?;
    part2(&ranges)?;

    part1_step(&ranges)?;
    part2_step(&ranges)?;
    Ok(())
}

#[test]
fn split_range_test() -> Result<()> {
    assert_eq!(split_range(90, 115), vec![(90, 99), (100, 115)]);
    assert_eq!(split_range(103, 115), vec![(103, 115)]);
    assert_eq!(
        split_range(90, 1215),
        vec![(90, 99), (100, 999), (1000, 1215)]
    );
    Ok(())
}

#[test]
fn find_invalid_test() -> Result<()> {
    assert_eq!(find_invalid(90, 99, 1), vec![99]);
    assert_eq!(find_invalid(101, 120, 1), vec![111]);
    assert_eq!(find_invalid(38593856, 38593862, 4), vec![38593859]);
    assert_eq!(find_invalid(11, 22, 1), vec![11, 22]);
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    let ranges = parse_input(input)?;
    assert_eq!(part1(&ranges).unwrap(), 1227775554);
    assert_eq!(part2(&ranges).unwrap(), 4174379265);

    assert_eq!(part1_step(&ranges).unwrap(), 1227775554);
    assert_eq!(part2_step(&ranges).unwrap(), 4174379265);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = include_str!("../input/input.txt");
    let ranges = parse_input(input)?;
    assert_eq!(part1(&ranges).unwrap(), 26255179562);
    assert_eq!(part2(&ranges).unwrap(), 31680313976);

    assert_eq!(part1_step(&ranges).unwrap(), 26255179562);
    assert_eq!(part2_step(&ranges).unwrap(), 31680313976);
    Ok(())
}
