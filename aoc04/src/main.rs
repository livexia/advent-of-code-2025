use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Vec<char>>> {
    Ok(input
        .as_ref()
        .trim()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().chars().collect())
        .collect())
}

fn adjacent(grid: &[Vec<char>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let (mx, my) = (grid.len() as isize, grid[0].len() as isize);
    let (x, y) = (x as isize, y as isize);
    let positions = [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ];
    positions
        .into_iter()
        .filter_map(|(x, y)| {
            if x >= 0 && x < mx && y >= 0 && y < my {
                Some((x as usize, y as usize))
            } else {
                None
            }
        })
        .collect()
}

fn part1(grid: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let count = grid
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter(|(j, c)| {
                    c == &&'@'
                        && adjacent(grid, i, *j)
                            .iter()
                            .filter(|(x, y)| grid[*x][*y] == '@')
                            .count()
                            < 4
                })
                .count()
        })
        .sum();

    println!("part1: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
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
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let grid = parse_input(input)?;
    assert_eq!(part1(&grid).unwrap(), 13);
    Ok(())
}
