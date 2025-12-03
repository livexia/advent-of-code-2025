use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Vec<usize>>> {
    Ok(input
        .as_ref()
        .trim()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| (c as u8 - b'0') as usize)
                .collect::<Vec<_>>()
        })
        .collect())
}

fn find_largest_joltage(battery: &[usize], number: usize) -> usize {
    let length = battery.len();
    let mut joltage = 0;
    let mut next_battery = 0;
    for l in (0..number).rev() {
        let mut max_battery = 0;
        (next_battery..(length - l)).for_each(|left| {
            if battery[left] > max_battery {
                max_battery = battery[left];
                next_battery = left + 1;
            }
        });
        joltage = joltage * 10 + max_battery;
    }
    joltage
}

fn part1(batteries: &[Vec<usize>]) -> Result<usize> {
    let _start = Instant::now();

    let joltage = batteries.iter().map(|b| find_largest_joltage(b, 2)).sum();

    println!("part 1: {joltage}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(joltage)
}

fn part2(batteries: &[Vec<usize>]) -> Result<usize> {
    let _start = Instant::now();

    let joltage = batteries.iter().map(|b| find_largest_joltage(b, 12)).sum();

    println!("part 2: {joltage}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(joltage)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let batteries = parse_input(input)?;

    part1(&batteries)?;
    part2(&batteries)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    let batteries = parse_input(input)?;
    assert_eq!(part1(&batteries).unwrap(), 357);
    assert_eq!(part2(&batteries).unwrap(), 3121910778619);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let batteries = parse_input(input)?;
    assert_eq!(part1(&batteries).unwrap(), 16927);
    assert_eq!(part2(&batteries).unwrap(), 167384358365132);
    Ok(())
}
