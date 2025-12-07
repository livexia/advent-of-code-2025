use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Grid = Vec<Vec<char>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<Grid> {
    Ok(input
        .as_ref()
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().chars().collect())
        .collect())
}

fn part1(grid: &Grid) -> Result<usize> {
    let _start = Instant::now();

    let mut count = 0;
    let mut beams: Vec<_> = grid[0].iter().map(|c| c == &'S').collect();

    for row in &grid[1..] {
        for j in 0..beams.len() {
            if beams[j] && row[j] == '^' {
                beams[j] = false;
                beams[j - 1] = true;
                beams[j + 1] = true;
                count += 1;
            }
        }
    }

    println!("part 1: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn dfs(current: (usize, usize), grid: &Grid, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(v) = cache.get(&current) {
        return *v;
    }
    let (x, y) = current;
    let c = match grid[x][y] {
        '.' | 'S' => {
            if x + 1 < grid.len() {
                dfs((x + 1, y), grid, cache)
            } else {
                1
            }
        }
        '^' => dfs((x, y - 1), grid, cache) + dfs((x, y + 1), grid, cache),
        _ => unreachable!(),
    };
    cache.insert(current, c);
    c
}

fn part2(grid: &Grid) -> Result<usize> {
    let _start = Instant::now();

    let start = grid[0].iter().position(|c| c == &'S').unwrap();

    let count = dfs((0, start), grid, &mut HashMap::new());

    println!("part 2: {count}");
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
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    let grid = parse_input(input)?;
    assert_eq!(part1(&grid).unwrap(), 21);
    assert_eq!(part2(&grid).unwrap(), 40);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();

    let grid = parse_input(input)?;
    assert_eq!(part1(&grid).unwrap(), 1681);
    assert_eq!(part2(&grid).unwrap(), 422102272495018);
    Ok(())
}
