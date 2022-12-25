use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::{rev, Itertools};
use petgraph::algo::toposort;
use petgraph::prelude::*;

fn solve_part_one(data: &[i32]) {
    let mut data = data.to_vec();
    data.push(0);
    data.sort_unstable();
    let max = *data.last().unwrap();
    data.push(max + 3);

    let diffs = data.windows(2).map(|w| w[1] - w[0]).collect_vec();
    println!("Sequence: {:?}", data);
    println!("Diffs: {:?}", diffs);
    let diffs1 = diffs.iter().filter(|&&x| x == 1).count();
    let diffs3 = diffs.iter().filter(|&&x| x == 3).count();
    println!("1-diffs: {}", diffs1);
    println!("3-diffs: {}", diffs3);
    println!("[part1] 1-diffs * 3-diffs = {}", diffs1 * diffs3);
}

fn solve_part_two(data: &[i32]) {
    let mut data = data.to_vec();
    let max = *data.iter().max().unwrap();
    let end = max + 3;
    data.push(0);
    data.push(end);
    data.sort_unstable();

    assert_eq!(data.iter().unique().count(), data.len());

    let mut graph = DiGraphMap::new();
    for comb in data.iter().combinations(2) {
        let a = *comb[0];
        let b = *comb[1];
        if (0..=3).contains(&(b - a)) {
            graph.add_edge(a, b, ());
        }
    }

    let topo = toposort(&graph, None).unwrap();
    let mut weights = HashMap::<_, u64>::new();
    weights.insert(end, 1);
    for v in rev(topo).skip(1) {
        let w = graph.neighbors(v).map(|u| weights[&u]).sum();
        println!("Weight of {} is {}", v, w);
        weights.insert(v, w);
    }
    println!("[part2] paths: {}", weights[&0]);
}

fn main() -> Result<()> {
    let path = "data/input.txt";
    let data = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec();

    solve_part_one(&data);
    println!();
    solve_part_two(&data);

    Ok(())
}
