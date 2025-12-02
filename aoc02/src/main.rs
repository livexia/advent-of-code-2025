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

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let ranges = parse_input(input)?;

    part1(&ranges)?;
    part2(&ranges)?;
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
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = include_str!("../input/input.txt");
    let ranges = parse_input(input)?;
    assert_eq!(part1(&ranges).unwrap(), 26255179562);
    assert_eq!(part2(&ranges).unwrap(), 31680313976);
    Ok(())
}
