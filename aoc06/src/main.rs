use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<(Vec<Vec<usize>>, Vec<String>)> {
    let mut nums: Vec<Vec<usize>> = Vec::new();
    let mut ops = Vec::new();

    for line in input.as_ref().trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let line: Vec<_> = line.split_whitespace().collect();
        if line[0].parse::<usize>().is_ok() {
            nums.push(
                line.iter()
                    .map(|n| n.parse::<usize>().map_err(|e| e.into()))
                    .collect::<Result<_>>()?,
            );
        } else {
            ops = line.iter().map(|c| c.to_string()).collect();
        }
    }

    Ok((nums, ops))
}

fn part1(nums: &[Vec<usize>], ops: &[String]) -> Result<usize> {
    let _start = Instant::now();

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

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (nums, ops) = parse_input(input)?;

    part1(&nums, &ops)?;
    // part2()?;
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
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (nums, ops) = parse_input(input)?;
    assert_eq!(part1(&nums, &ops).unwrap(), 5977759036837);
    Ok(())
}
