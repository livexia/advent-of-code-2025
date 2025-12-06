use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<(Vec<String>, Vec<String>)> {
    let lines: Vec<_> = input
        .as_ref()
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    let ops = lines[lines.len() - 1]
        .split_whitespace()
        .map(|c| c.to_string())
        .collect();

    let nums: Vec<_> = lines[..lines.len() - 1]
        .iter()
        .map(|l| l.to_string())
        .collect();

    Ok((nums, ops))
}

fn part1(nums: &[String], ops: &[String]) -> Result<usize> {
    let _start = Instant::now();

    let nums: Vec<Vec<_>> = nums
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut ans = 0;
    for (i, op) in ops.iter().enumerate() {
        if op == "+" {
            ans += nums.iter().map(|n| n[i]).sum::<usize>()
        } else {
            ans += nums.iter().map(|n| n[i]).product::<usize>()
        }
    }

    println!("part 1: {ans}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(ans)
}

fn part2<T: AsRef<str>>(input: T) -> Result<usize> {
    let _start = Instant::now();

    let mut ans = 0;
    let lines: Vec<Vec<_>> = input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.bytes().rev().collect())
        .collect();
    let op_row = lines.len() - 1;
    let mut reals = vec![];

    for i in 0..lines[0].len() {
        let real = lines[0..op_row]
            .iter()
            .filter_map(|row| {
                if row[i] == b' ' {
                    None
                } else {
                    Some((row[i] - b'0') as usize)
                }
            })
            .fold(0, |r, n| r * 10 + n);
        match lines[op_row][i] {
            b'+' => ans += reals.iter().sum::<usize>() + real,
            b'*' => ans += reals.iter().product::<usize>() * real,
            _ => {
                if real == 0 {
                    reals.clear();
                } else {
                    reals.push(real);
                }
            }
        }
    }

    println!("part 2: {ans}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(ans)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (nums, ops) = parse_input(&input)?;

    part1(&nums, &ops)?;
    part2(&input)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    let (nums, ops) = parse_input(input)?;
    assert_eq!(part1(&nums, &ops).unwrap(), 4277556);
    assert_eq!(part2(input).unwrap(), 3263827);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (nums, ops) = parse_input(&input)?;
    assert_eq!(part1(&nums, &ops).unwrap(), 5977759036837);
    assert_eq!(part2(&input).unwrap(), 9630000828442);
    Ok(())
}
