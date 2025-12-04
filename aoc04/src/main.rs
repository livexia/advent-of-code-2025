use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};
use std::isize;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<HashSet<(isize, isize)>> {
    let mut grid = HashSet::new();

    for (i, line) in input.as_ref().trim().lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            if c == '@' {
                grid.insert((i as isize, j as isize));
            }
        }
    }

    Ok(grid)
}

fn adjacent(p: (isize, isize)) -> [(isize, isize); 8] {
    let (x, y) = p;
    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

fn part1(grid: &HashSet<(isize, isize)>) -> Result<usize> {
    let _start = Instant::now();

    let count = grid
        .iter()
        .filter(|&p| adjacent(*p).iter().filter(|&p| grid.contains(p)).count() < 4)
        .count();

    println!("part1: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn part2(grid: &HashSet<(isize, isize)>) -> Result<usize> {
    let _start = Instant::now();

    let mut flag = true;
    let mut count = 0;
    let mut grid = grid.clone();

    while flag {
        flag = false;
        let mut temp = HashSet::new();

        for &p in &grid {
            if adjacent(p).iter().filter(|&p| grid.contains(p)).count() < 4 {
                flag = true;
            } else {
                temp.insert(p);
            }
        }
        count += grid.len() - temp.len();
        grid = temp;
    }

    println!("part2: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let grid = parse_input(input)?;

    part1(&grid)?;
    part2(&grid)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let grid = parse_input(input)?;
    assert_eq!(part1(&grid).unwrap(), 13);
    assert_eq!(part2(&grid).unwrap(), 43);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let grid = parse_input(input)?;
    assert_eq!(part1(&grid).unwrap(), 1419);
    assert_eq!(part2(&grid).unwrap(), 8739);
    Ok(())
}
