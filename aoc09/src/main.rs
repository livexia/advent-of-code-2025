use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

use rayon::prelude::*;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Point = (i128, i128);

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Point>> {
    input
        .as_ref()
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            if let Some((x, y)) = l.split_once(",") {
                Ok((x.parse().unwrap(), y.parse().unwrap()))
            } else {
                err!("unable to parse: {l:?}")
            }
        })
        .collect()
}

fn area(p: Point, other: Point) -> u128 {
    (1 + p.0.abs_diff(other.0)) * (p.1.abs_diff(other.1) + 1)
}

fn part1(grid: &[Point]) -> Result<u128> {
    let _start = Instant::now();

    let mut largest = 0;
    for i in 0..grid.len() {
        for j in i + 1..grid.len() {
            largest = largest.max(area(grid[i], grid[j]))
        }
    }

    println!("part 1: {largest}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(largest)
}

fn cross_product(p: Point, a: Point, b: Point) -> i128 {
    (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0)
}

fn is_on_segment(p: Point, a: Point, b: Point) -> bool {
    let min_x = a.0.min(b.0);
    let max_x = a.0.max(b.0);
    let min_y = a.1.min(b.1);
    let max_y = a.1.max(b.1);
    if p.0 < min_x || p.0 > max_x || p.1 < min_y || p.1 > max_y {
        return false;
    }

    cross_product(p, a, b) == 0
}

fn is_in_polygon(p: Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let pi = polygon[i];
        let pj = polygon[j];

        if is_on_segment(p, pi, pj) {
            return true;
        }

        if (pi.1 > p.1) != (pj.1 > p.1) {
            let det = (pj.0 - pi.0) * (p.1 - pi.1) - (p.0 - pi.0) * (pj.1 - pi.1);
            if pj.1 > pi.1 {
                if det > 0 {
                    inside = !inside;
                }
            } else if det < 0 {
                inside = !inside;
            }
        }

        j = i;
    }
    inside
}

fn is_proper_intersection(a: Point, b: Point, c: Point, d: Point) -> bool {
    let d1 = cross_product(a, c, d);
    let d2 = cross_product(b, c, d);
    let d3 = cross_product(c, a, b);
    let d4 = cross_product(d, a, b);

    ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))
}

#[allow(unused)]
fn is_segment_in_polygon(p1: Point, p2: Point, polygon: &[Point]) -> bool {
    if !is_in_polygon(p1, polygon) || !is_in_polygon(p2, polygon) {
        return false;
    }

    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let poly_p1 = polygon[i];
        let poly_p2 = polygon[j];
        if is_proper_intersection(p1, p2, poly_p1, poly_p2) {
            return false;
        }
        j = i;
    }
    true
}

/// Example of failure:
/// A "U" shaped polygon where the rectangle fills the gap of the "U" perfectly.
/// Since the rectangle edges coincide with the polygon's "outer" edges without crossing them strictly,
/// this function will return `true` (valid), even though the rectangle is topologically outside.
#[allow(dead_code)]
fn is_rect_in_polygon(p1: Point, p2: Point, polygon: &[Point]) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let a = (x1.min(x2), y1.min(y2));
    let b = (x1.min(x2), y1.max(y2));
    let c = (x1.max(x2), y1.min(y2));
    let d = (x1.max(x2), y1.max(y2));
    if !is_in_polygon(a, polygon)
        || !is_in_polygon(b, polygon)
        || !is_in_polygon(c, polygon)
        || !is_in_polygon(d, polygon)
    {
        return false;
    }

    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let poly_p1 = polygon[i];
        let poly_p2 = polygon[j];
        if is_proper_intersection(a, b, poly_p1, poly_p2)
            || is_proper_intersection(b, c, poly_p1, poly_p2)
            || is_proper_intersection(c, d, poly_p1, poly_p2)
            || is_proper_intersection(d, a, poly_p1, poly_p2)
        {
            return false;
        }
        j = i;
    }
    true
}

#[allow(dead_code)]
fn is_rect_in_polygon_with_aabb(p1: Point, p2: Point, polygon: &[Point]) -> bool {
    // **DEPRECATED (Known Issues):** This function currently uses an incorrect
    // method for polygon filling, as it fails to prevent external rectangles
    // from being mistakenly included in concave gaps.
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let rx_min = x1.min(x2);
    let rx_max = x1.max(x2);
    let ry_min = y1.min(y2);
    let ry_max = y1.max(y2);

    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let poly_p1 = polygon[i];
        let poly_p2 = polygon[j];
        let px_min = poly_p1.0.min(poly_p2.0);
        let px_max = poly_p1.0.max(poly_p2.0);
        let py_min = poly_p1.1.min(poly_p2.1);
        let py_max = poly_p1.1.max(poly_p2.1);

        if px_min == px_max {
            let p_x = px_min;
            if p_x > rx_min && p_x < rx_max && !(py_max <= ry_min || py_min >= ry_max) {
                return false;
            }
        }

        if py_min == py_max {
            let p_y = py_min;
            if p_y > ry_min && p_y < ry_max && !(px_max <= rx_min || px_min >= rx_max) {
                return false;
            }
        }

        j = i;
    }
    true
}

fn part2(grid: &[Point]) -> Result<u128> {
    let _start = Instant::now();

    let largest = (0..grid.len())
        .into_par_iter()
        .map(|i| {
            let mut local_largest = 0;
            for j in i + 1..grid.len() {
                if area(grid[i], grid[j]) <= local_largest {
                    continue;
                }
                if is_rect_in_polygon(grid[i], grid[j], grid) {
                    local_largest = local_largest.max(area(grid[i], grid[j]));
                }
            }
            local_largest
        })
        .max()
        .unwrap_or(0);

    println!("part 2: {largest}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(largest)
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
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    let grid = parse_input(input)?;
    // assert_eq!(part1(&grid).unwrap(), 50);
    assert_eq!(part2(&grid).unwrap(), 24);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let grid = parse_input(input)?;
    assert_eq!(part1(&grid).unwrap(), 4765757080);
    assert_eq!(part2(&grid).unwrap(), 1498673376);
    Ok(())
}
