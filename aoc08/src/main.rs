use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (isize, isize, isize);

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Coord>> {
    input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut it = l.trim().split(",");
            match (it.next(), it.next(), it.next()) {
                (Some(x), Some(y), Some(z)) => Ok((
                    x.parse::<isize>().unwrap(),
                    y.parse::<isize>().unwrap(),
                    z.parse::<isize>().unwrap(),
                )),
                _ => err!("unable parse input: {l:?}"),
            }
        })
        .collect()
}

fn distance(c: Coord, other: Coord) -> usize {
    let dx = c.0.abs_diff(other.0);
    let dy = c.1.abs_diff(other.1);
    let dz = c.2.abs_diff(other.2);
    dx.pow(2) + dy.pow(2) + dz.pow(2)
}

fn find(circuits: &mut [usize], i: usize) -> usize {
    if circuits[i] == circuits.len() {
        circuits[i] = i;
        return i;
    }
    if circuits[i] != i {
        circuits[i] = find(circuits, circuits[i]);
    }
    circuits[i]
}

fn part1(coords: &[Coord], pairs: usize) -> Result<usize> {
    let _start = Instant::now();

    let mut edges = vec![];
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            edges.push((i, j));
        }
    }

    edges.sort_by_key(|(a, b)| distance(coords[*a], coords[*b]));

    let mut circuits: Vec<_> = vec![coords.len(); coords.len()];

    for &(i, j) in edges.iter().take(pairs) {
        let i_root = find(&mut circuits, i);
        let j_root = find(&mut circuits, j);
        if i_root != j_root {
            circuits[i_root] = j_root;
        }
    }

    let mut count = HashMap::new();

    for i in 0..circuits.len() {
        if circuits[i] == circuits.len() {
            continue;
        }
        let e = count.entry(find(&mut circuits, i)).or_insert(0);
        *e += 1;
    }

    let mut count: Vec<_> = count.values().collect();
    count.sort_by(|a, b| b.cmp(a));
    let size = count[0] * count[1] * count[2];

    println!("part 1: {size}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(size)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let coords = parse_input(input)?;

    part1(&coords, 1000)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    let coords = parse_input(input)?;
    assert_eq!(part1(&coords, 10).unwrap(), 40);
    // assert_eq!(part2(&coords).unwrap(), 25272);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let coords = parse_input(input)?;
    assert_eq!(part1(&coords, 1000).unwrap(), 97384);
    // assert_eq!(part2(&coords).unwrap(), 25272);
    Ok(())
}
