use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (usize, usize, usize);
type Edge = (usize, usize);

fn parse_input<T: AsRef<str>>(input: T) -> Result<(Vec<Coord>, Vec<Edge>)> {
    let coords: Vec<Coord> = input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut it = l.trim().split(",");
            match (it.next(), it.next(), it.next()) {
                (Some(x), Some(y), Some(z)) => Ok((
                    x.parse::<usize>().unwrap(),
                    y.parse::<usize>().unwrap(),
                    z.parse::<usize>().unwrap(),
                )),
                _ => err!("unable parse input: {l:?}"),
            }
        })
        .collect::<Result<_>>()?;
    let connections = generate_connections(&coords);
    Ok((coords, connections))
}

fn generate_connections(coords: &[Coord]) -> Vec<Edge> {
    let mut edges = vec![];
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            edges.push((i, j));
        }
    }

    edges.sort_unstable_by_key(|(a, b)| distance(coords[*a], coords[*b]));
    edges
}

fn distance(c: Coord, other: Coord) -> usize {
    let dx = c.0.abs_diff(other.0);
    let dy = c.1.abs_diff(other.1);
    let dz = c.2.abs_diff(other.2);
    dx.pow(2) + dy.pow(2) + dz.pow(2)
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            return true;
        }
        false
    }

    fn get_size(&mut self, i: usize) -> usize {
        let root = self.find(i);
        self.size[root]
    }
}

fn part1(coords: &[Coord], conns: &[Edge], pairs: usize) -> Result<usize> {
    let _start = Instant::now();

    let n = coords.len();
    let mut uf = UnionFind::new(n);
    for &(u, v) in conns.iter().take(pairs) {
        uf.union(u, v);
    }

    let mut sizes: Vec<_> = (0..n)
        .filter(|&i| uf.parent[i] == i)
        .map(|i| uf.size[i])
        .collect();

    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let size = sizes[0] * sizes[1] * sizes[2];

    println!("part 1: {size}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(size)
}

fn part2(coords: &[Coord], conns: &[Edge]) -> Result<usize> {
    let _start = Instant::now();

    let n = coords.len();
    let mut ans = 0;
    let mut uf = UnionFind::new(n);
    for &(u, v) in conns.iter() {
        uf.union(u, v);

        if uf.get_size(v) == n {
            ans = coords[u].0 * coords[v].0;
            break;
        }
    }

    println!("part 2: {ans}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(ans)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (coords, conns) = parse_input(input)?;

    part1(&coords, &conns, 1000)?;
    part2(&coords, &conns)?;
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
    let (coords, conns) = parse_input(input)?;
    assert_eq!(part1(&coords, &conns, 10).unwrap(), 40);
    assert_eq!(part2(&coords, &conns).unwrap(), 25272);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (coords, conns) = parse_input(input)?;
    assert_eq!(part1(&coords, &conns, 1000).unwrap(), 97384);
    assert_eq!(part2(&coords, &conns).unwrap(), 9003685096);
    Ok(())
}
