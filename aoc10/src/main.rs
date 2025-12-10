use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

use good_lp::{Expression, Solution, SolverModel, Variable, default_solver, variable, variables};

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
    fn min_presses_for_lights(&self) -> Option<usize> {
        // BFS

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

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
                        queue.push_back((next_lights, button_pressed | 1 << index, presses + 1));
                    } else {
                        return Some(presses + 1);
                    }
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn min_presses_for_joltage_bfs(&self) -> Option<usize> {
        // BFS

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((self.joltage.clone(), vec![None; self.buttons.len()], 0));

        while let Some((counter, button_pressed, presses)) = queue.pop_front() {
            if visited.insert(button_pressed.to_vec()) {
                for (index, button) in self.buttons.iter().enumerate() {
                    if button_pressed[index].is_some() {
                        continue;
                    }
                    let max_p = max_press(&counter, button);
                    for p in 0..=max_p {
                        let mut new_counter = counter.to_vec();
                        let mut new_button_pressed = button_pressed.to_vec();
                        new_button_pressed[index] = Some(p);
                        for &b in button {
                            new_counter[b] -= p;
                        }
                        if new_counter.iter().all(|j| j == &0) {
                            return Some(presses + p);
                        } else {
                            queue.push_back((
                                new_counter.to_vec(),
                                new_button_pressed.to_vec(),
                                presses + p,
                            ));
                        }
                    }
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn min_presses_for_joltage_dfs(
        &self,
        counter: &[usize],
        button_pressed: &[Option<usize>],
        cache: &mut HashMap<Vec<Option<usize>>, usize>,
    ) -> Option<usize> {
        if counter.iter().all(|j| j == &0) {
            return Some(0);
        }
        if button_pressed.iter().all(|b| b.is_some()) {
            return None;
        }
        if let Some(p) = cache.get(button_pressed) {
            if *p == usize::MAX {
                return None;
            }
            return Some(*p);
        }
        let mut min_presses = usize::MAX;
        for (b_index, button) in self.buttons.iter().enumerate() {
            if button_pressed[b_index].is_some() {
                // pressed
                continue;
            }
            let max_p = max_press(counter, button);
            for p in 0..=max_p {
                let mut new_counter = counter.to_vec();
                let mut new_button_pressed = button_pressed.to_vec();
                new_button_pressed[b_index] = Some(p);
                for &b in button {
                    new_counter[b] -= p;
                }
                if let Some(pressed) =
                    self.min_presses_for_joltage_dfs(&new_counter, &new_button_pressed, cache)
                {
                    min_presses = min_presses.min(p + pressed);
                }
            }
        }
        cache.insert(button_pressed.to_vec(), min_presses);
        if min_presses != usize::MAX {
            Some(min_presses)
        } else {
            None
        }
    }

    fn min_presses_for_joltage_good_lp(&self) -> Option<usize> {
        let f_count = self.joltage.len();
        let b_count = self.buttons.len();
        let mut f = vec![vec![0; b_count]; f_count];
        for (index, button) in self.buttons.iter().enumerate() {
            for &b in button {
                f[b][index] = 1;
            }
        }
        let mut problem = variables!();
        let vars = vec![variable().min(0).integer(); b_count];
        let t: Vec<Variable> = problem.add_all(vars);
        let objective: Expression = t.iter().sum();
        let mut model = problem.minimise(&objective).using(default_solver);
        model.set_parameter("verbose", "false");

        for (row, &j) in f.iter().zip(&self.joltage) {
            let mut constraint: Expression = Expression::from(0);
            for (&coeff, &var) in row.iter().zip(&t) {
                if coeff == 1 {
                    constraint += var;
                }
            }
            model = model.with(constraint.eq(j as f64))
        }
        match model.solve() {
            Ok(sol) => Some(sol.eval(objective).round() as usize),
            Err(e) => {
                println!("Solver error: {e:?}");
                None
            }
        }
    }
}

#[allow(dead_code)]
fn max_press(joltage: &[usize], button: &[usize]) -> usize {
    let mut m = usize::MAX;
    for &b in button.iter() {
        m = m.min(joltage[b]);
    }
    m
}

fn part1(machines: &[Machine]) -> Result<usize> {
    let _start = Instant::now();

    let ans = machines
        .iter()
        .filter_map(|m| m.min_presses_for_lights())
        .sum();

    println!("part 1: {ans}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(ans)
}

fn part2(machines: &[Machine]) -> Result<usize> {
    let _start = Instant::now();

    let ans = machines
        .iter()
        .filter_map(|m| m.min_presses_for_joltage_good_lp())
        .sum();

    println!("part 2: {ans}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(ans)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let machines = parse_input(input)?;

    part1(&machines)?;
    part2(&machines)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    let machines = parse_input(input)?;
    assert_eq!(part1(&machines).unwrap(), 7);
    assert_eq!(part2(&machines).unwrap(), 33);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let machines = parse_input(input)?;
    assert_eq!(part1(&machines).unwrap(), 488);
    assert_eq!(part2(&machines).unwrap(), 18771);
    Ok(())
}
