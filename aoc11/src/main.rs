use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Connections {
    index_table: HashMap<usize, String>,
    device_table: HashMap<String, usize>,
    connection_table: HashMap<usize, Vec<usize>>,
}

impl Connections {
    fn new() -> Self {
        Self {
            index_table: HashMap::new(),
            device_table: HashMap::new(),
            connection_table: HashMap::new(),
        }
    }

    fn adding_connection(&mut self, connc: &str) -> Result<()> {
        if let Some((input, outputs)) = connc.split_once(":") {
            let input_id = self.insert_device(input.trim());
            let output_ids = outputs
                .split_whitespace()
                .map(|d| self.insert_device(d))
                .collect();
            self.connection_table.insert(input_id, output_ids);
            return Ok(());
        }
        err!("unable to parse connection: {connc:?}")
    }

    fn insert_device(&mut self, dev: &str) -> usize {
        if let Some(id) = self.device_table.get(dev) {
            *id
        } else {
            let id = self.index_table.len();
            self.index_table.insert(id, dev.to_string());
            self.device_table.insert(dev.to_string(), id);
            id
        }
    }

    #[allow(dead_code)]
    fn get_device(&self, id: usize) -> Option<&String> {
        self.index_table.get(&id)
    }

    fn get_id(&self, dev: &str) -> Option<&usize> {
        self.device_table.get(dev)
    }

    fn get_outputs(&self, id: usize) -> Option<&Vec<usize>> {
        self.connection_table.get(&id)
    }
}
fn parse_input<T: AsRef<str>>(input: T) -> Result<Connections> {
    let mut connections = Connections::new();
    for line in input.as_ref().lines().filter(|l| !l.trim().is_empty()) {
        connections.adding_connection(line)?;
    }
    Ok(connections)
}

fn count_path_dfs(
    current: usize,
    target: usize,
    connections: &Connections,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if current == target {
        return 1;
    }
    if let Some(count) = cache.get(&current) {
        return *count;
    }

    let mut count = 0;
    if let Some(outputs) = connections.get_outputs(current) {
        for &next in outputs {
            count += count_path_dfs(next, target, connections, cache);
        }
    }
    cache.insert(current, count);
    count
}

fn part1(connections: &Connections) -> Result<usize> {
    let _start = Instant::now();

    let &you_id = connections
        .get_id("you")
        .ok_or("unable to find device with name: you")?;
    let &out_id = connections
        .get_id("out")
        .ok_or("unable to find device with name: out")?;

    let count = count_path_dfs(you_id, out_id, connections, &mut HashMap::new());

    println!("part 1: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn part2_dfs(
    current: usize,
    target: usize,
    visited: u8,
    dac_fft: &[usize],
    connections: &Connections,
    cache: &mut HashMap<(usize, u8), usize>,
) -> usize {
    if visited == 3 && current == target {
        return 1;
    }
    if let Some(count) = cache.get(&(current, visited)) {
        return *count;
    }
    let mut count = 0;
    if let Some(outputs) = connections.get_outputs(current) {
        for &next in outputs {
            let next_visited = visited
                | if next == dac_fft[0] {
                    1
                } else if next == dac_fft[1] {
                    2
                } else {
                    0
                };
            count += part2_dfs(next, target, next_visited, dac_fft, connections, cache);
        }
    }
    cache.insert((current, visited), count);
    count
}

fn part2(connections: &Connections) -> Result<usize> {
    let _start = Instant::now();

    let &svr_id = connections
        .get_id("svr")
        .ok_or("unable to find device with name: svr")?;
    let &out_id = connections
        .get_id("out")
        .ok_or("unable to find device with name: out")?;
    let dac_fft: Vec<_> = ["dac", "fft"]
        .iter()
        .map(|d| *connections.get_id(d).unwrap())
        .collect();
    println!("{dac_fft:?}");

    let count = part2_dfs(
        svr_id,
        out_id,
        0,
        &dac_fft,
        connections,
        &mut HashMap::new(),
    );

    println!("part 2: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let connections = parse_input(input)?;
    part1(&connections)?;
    part2(&connections)?;
    Ok(())
}

#[test]
fn example_input_part1() -> Result<()> {
    let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    let connections = parse_input(input)?;
    assert_eq!(part1(&connections).unwrap(), 5);
    Ok(())
}

#[test]
fn example_input_part2() -> Result<()> {
    let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
    let connections = parse_input(input)?;
    assert_eq!(part2(&connections).unwrap(), 2);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let connections = parse_input(input)?;
    assert_eq!(part1(&connections).unwrap(), 658);
    assert_eq!(part2(&connections).unwrap(), 371113003846800);
    Ok(())
}
