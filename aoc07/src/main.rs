use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Grid = HashMap<(isize, isize), char>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<Grid> {
    let mut grid = Grid::new();
    for (i, line) in input
        .as_ref()
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
    {
        for (j, c) in line.trim().chars().enumerate() {
            grid.insert((i as isize, j as isize), c);
        }
    }
    Ok(grid)
}

fn part1(grid: &Grid) -> Result<usize> {
    let _start = Instant::now();

    let mut count = 0;

    let &start = grid.iter().find(|(_, c)| c == &&'S').unwrap().0;

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut visited = HashSet::new();

    while let Some(current) = queue.pop_front() {
        if !visited.insert(current) {
            continue;
        }
        let (x, y) = current;
        match grid.get(&current) {
            Some('.') | Some('S') => queue.push_back((x + 1, y)),
            Some('^') => {
                count += 1;
                for p in [(x, y - 1), (x, y + 1)] {
                    queue.push_back(p);
                }
            }
            None => continue,
            _ => unreachable!(),
        }
    }

    println!("part 1: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn dfs(current: (isize, isize), grid: &Grid, cache: &mut HashMap<(isize, isize), usize>) -> usize {
    if let Some(v) = cache.get(&current) {
        return *v;
    }
    let c = match grid.get(&current) {
        Some('.') | Some('S') => dfs((current.0 + 1, current.1), grid, cache),
        Some('^') => {
            let (x, y) = current;
            dfs((x, y - 1), grid, cache) + dfs((x, y + 1), grid, cache)
        }
        None => 1,
        _ => unreachable!(),
    };
    cache.insert(current, c);
    c
}

fn part2(grid: &Grid) -> Result<usize> {
    let _start = Instant::now();

    let &start = grid.iter().find(|(_, c)| c == &&'S').unwrap().0;

    let count = dfs(start, grid, &mut HashMap::new());

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
