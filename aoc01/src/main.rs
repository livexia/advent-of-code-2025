use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<i32>> {
    input
        .as_ref()
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            if let Some(n) = l.trim().strip_prefix("R") {
                n.parse::<i32>()
            } else if let Some(n) = l.trim().strip_prefix("L") {
                n.parse::<i32>().map(|val| -val)
            } else {
                return err!("Invalid line format: missing 'R' or 'L' prefix: {l:?}");
            }
            .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
        .collect()
}

fn part1(rotations: &[i32]) -> Result<u32> {
    let _start = Instant::now();

    let mut dial = 50;
    let mut password = 0;

    for rot in rotations {
        dial = (dial + rot).rem_euclid(100);
        password += (dial == 0) as u32;
    }

    println!("part1: {password}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(password)
}

fn part2(rotations: &[i32]) -> Result<u32> {
    let _start = Instant::now();

    let mut dial = 50;
    let mut password = 0;

    for rot in rotations {
        password += rot.unsigned_abs() / 100;
        let rot = rot % 100;
        let temp = dial + rot;
        password += (dial != 100 && temp >= 100) as u32 + (dial != 0 && temp <= 0) as u32;

        dial = temp.rem_euclid(100);
    }

    println!("part2: {password}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(password)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let rotations = parse_input(input)?;

    part1(&rotations)?;
    part2(&rotations)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let rotations = parse_input(input)?;
    assert_eq!(part1(&rotations).unwrap(), 3);
    assert_eq!(part2(&rotations).unwrap(), 6);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = include_str!("../input/input.txt");
    let rotations = parse_input(input)?;
    assert_eq!(part1(&rotations).unwrap(), 1100);
    assert_eq!(part2(&rotations).unwrap(), 6358);
    Ok(())
}
