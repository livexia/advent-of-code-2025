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
    device_table: HashMap<String, usize>,
    adj_list: Vec<Vec<usize>>,
}

impl Connections {
    fn new() -> Self {
        Self {
            device_table: HashMap::new(),
            adj_list: Vec::new(),
        }
    }

    fn add_connection(&mut self, line: &str) -> Result<()> {
        if let Some((input, outputs)) = line.split_once(":") {
            let input_id = self.insert_device(input.trim());
            let output_ids = outputs
                .split_whitespace()
                .map(|d| self.insert_device(d))
                .collect();
            if input_id >= self.adj_list.len() {
                self.adj_list.resize(input_id + 1, vec![]);
            }
            self.adj_list[input_id] = output_ids;
            return Ok(());
        }
        err!("unable to parse connection: {line:?}")
    }

    fn insert_device(&mut self, dev: &str) -> usize {
        if let Some(id) = self.device_table.get(dev) {
            *id
        } else {
            let id = self.device_table.len();
            self.device_table.insert(dev.to_string(), id);
            id
        }
    }

    fn get_id(&self, dev: &str) -> Option<usize> {
        self.device_table.get(dev).copied()
    }

    fn get_outputs(&self, id: usize) -> &[usize] {
        if id < self.adj_list.len() {
            &self.adj_list[id]
        } else {
            &[]
        }
    }
}
fn parse_input<T: AsRef<str>>(input: T) -> Result<Connections> {
    let mut connections = Connections::new();
    for line in input.as_ref().lines().filter(|l| !l.trim().is_empty()) {
        connections.add_connection(line)?;
    }
    Ok(connections)
}

fn count_paths_dfs(
    current: usize,
    target: usize,
    connections: &Connections,
    cache: &mut [Option<usize>],
) -> usize {
    if current == target {
        return 1;
    }
    if let Some(count) = cache[current] {
        return count;
    }

    let mut count = 0;
    for &next in connections.get_outputs(current) {
        count += count_paths_dfs(next, target, connections, cache);
    }
    cache[current] = Some(count);
    count
}

fn part1(connections: &Connections) -> Result<usize> {
    let _start = Instant::now();

    let you = connections.get_id("you").ok_or("node 'you' not found")?;
    let out = connections.get_id("out").ok_or("node 'out' not found")?;

    let count = count_paths_dfs(
        you,
        out,
        connections,
        &mut vec![None; connections.device_table.len()],
    );

    println!("part 1: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn count_paths_with_dac_fft(
    current: usize,
    target: usize,
    visited_mask: u8,
    dac_fft: &[usize],
    connections: &Connections,
    cache: &mut [Option<usize>],
) -> usize {
    if visited_mask == 3 && current == target {
        return 1;
    }
    if let Some(count) = cache[current * 4 + visited_mask as usize] {
        return count;
    }
    let mut count = 0;
    for &next in connections.get_outputs(current) {
        let next_mask = visited_mask
            | if next == dac_fft[0] {
                1
            } else if next == dac_fft[1] {
                2
            } else {
                0
            };
        count += count_paths_with_dac_fft(next, target, next_mask, dac_fft, connections, cache);
    }
    cache[current * 4 + visited_mask as usize] = Some(count);
    count
}

fn part2(connections: &Connections) -> Result<usize> {
    let _start = Instant::now();

    let svr = connections.get_id("svr").ok_or("node 'svr' not found")?;
    let out = connections.get_id("out").ok_or("node 'out' not found")?;
    let dac = connections.get_id("dac").ok_or("node 'dac' not found")?;
    let fft = connections.get_id("fft").ok_or("node 'fft' not found")?;

    let count = count_paths_with_dac_fft(
        svr,
        out,
        0,
        &[dac, fft],
        connections,
        &mut vec![None; connections.device_table.len() * 4],
    );

    println!("part 2: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn part2_segmented(connections: &Connections) -> Result<usize> {
    let _start = Instant::now();

    let svr = connections.get_id("svr").ok_or("node 'svr' not found")?;
    let out = connections.get_id("out").ok_or("node 'out' not found")?;
    let dac = connections.get_id("dac").ok_or("node 'dac' not found")?;
    let fft = connections.get_id("fft").ok_or("node 'fft' not found")?;

    let count_between = |start, end| {
        count_paths_dfs(
            start,
            end,
            connections,
            &mut vec![None; connections.device_table.len()],
        )
    };

    let mut count = 0;
    // svr -> dac -> fft -> out
    let dac_fft = count_between(dac, fft);
    if dac_fft != 0 {
        let svr_dac = count_between(svr, dac);
        let fft_out = count_between(fft, out);
        count = svr_dac * dac_fft * fft_out;
    }
    // svt -> fft -> dac -> out
    let fft_dac = count_between(fft, dac);
    if fft_dac != 0 {
        let svr_fft = count_between(svr, fft);
        let dac_out = count_between(dac, out);
        count += svr_fft * fft_dac * dac_out;
    }
    println!("part 2 with count segmented paths: {count}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(count)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let connections = parse_input(input)?;
    part1(&connections)?;
    part2(&connections)?;
    part2_segmented(&connections)?;
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
    assert_eq!(part2_segmented(&connections).unwrap(), 2);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let connections = parse_input(input)?;
    assert_eq!(part1(&connections).unwrap(), 658);
    assert_eq!(part2(&connections).unwrap(), 371113003846800);
    assert_eq!(part2_segmented(&connections).unwrap(), 371113003846800);
    Ok(())
}
