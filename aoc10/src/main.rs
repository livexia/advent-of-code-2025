use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Machine {
    lights: u128,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl FromStr for Machine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lights = 0;
        let mut buttons = vec![];
        let mut joltage = vec![];
        for p in s.split_whitespace() {
            if let Some(p) = p.strip_prefix("[") {
                let p = p
                    .strip_suffix("]")
                    .ok_or_else(|| format!("uable to parse lights for machine: {s:?}"))?;
                for (i, c) in p.trim().chars().enumerate() {
                    if c == '#' {
                        lights |= 1 << i;
                    }
                }
            } else if let Some(p) = p.strip_prefix("(") {
                let p = p
                    .strip_suffix(")")
                    .ok_or_else(|| format!("uable to parse buttons for machine: {s:?}"))?;
                buttons.push(
                    p.trim()
                        .split(",")
                        .map(|n| n.parse::<usize>().map_err(|e| e.into()))
                        .collect::<Result<_>>()?,
                );
            } else if let Some(p) = p.strip_prefix("{") {
                let p = p
                    .strip_suffix("}")
                    .ok_or_else(|| format!("uable to parse joltage for machine: {s:?}"))?;
                joltage = p
                    .trim()
                    .split(",")
                    .map(|n| n.parse::<usize>().map_err(|e| e.into()))
                    .collect::<Result<_>>()?;
            }
        }
        Ok(Self {
            lights,
            buttons,
            joltage,
        })
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Machine>> {
    input
        .as_ref()
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse())
        .collect()
}

fn press_button(mut lights: u128, button: &[usize]) -> u128 {
    for b in button {
        let mask = 1 << b;
        lights ^= mask;
    }
    lights
}

impl Machine {
    fn fewest_button_presses(&self) -> Option<usize> {
        // BFS

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let button_mask = (0..self.buttons.len()).fold(0, |m, i| m | 1 << i);

        queue.push_back((self.lights, 0, 0));

        while let Some((current_lights, button_pressed, presses)) = queue.pop_front() {
            if visited.insert(button_pressed) {
                for (index, button) in self.buttons.iter().enumerate() {
                    if button_pressed & 1 << index == 1 {
                        continue;
                    }
                    let next_lights = press_button(current_lights, button);
                    let button_pressed = button_pressed | 1 << index;
                    if next_lights != 0 {
                        if button_pressed == button_mask {
                            continue;
                        }
                        queue.push_back((next_lights, button_pressed | 1 << index, presses + 1));
                    } else {
                        return Some(presses + 1);
                    }
                }
            }
        }

        None
    }
}

fn part1(machines: &[Machine]) -> Result<usize> {
    let _start = Instant::now();

    let ans = machines
        .iter()
        .filter_map(|m| m.fewest_button_presses())
        .sum();

    println!("part 1: {ans}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(ans)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let machines = parse_input(input)?;

    part1(&machines)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    let machines = parse_input(input)?;
    assert_eq!(part1(&machines).unwrap(), 7);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let machines = parse_input(input)?;
    assert_eq!(part1(&machines).unwrap(), 488);
    Ok(())
}
