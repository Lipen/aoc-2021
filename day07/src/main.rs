use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Result};
use itertools::{rev, Itertools};
use once_cell_regex::regex;
use petgraph::dot::Dot;
use petgraph::prelude::*;
use petgraph::visit::{IntoNeighbors, Reversed};

fn main() -> Result<()> {
    let path = "data/input.txt";
    let mut storage: HashMap<(String, String), usize> = HashMap::new();
    BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .for_each(|line| {
            let re =
                regex!(r"^(\w+ \w+) bags? contain (\d+ \w+ \w+ bags?(?:, \d+ \w+ \w+ bags?)*)\.$");
            if let Some(caps) = re.captures(&line) {
                let lhs = caps.get(1).unwrap().as_str();
                let rhs = caps.get(2).unwrap().as_str();
                for item in rhs.split(", ") {
                    if let Some(caps) = regex!(r"(\d+) (\w+ \w+)").captures(&item) {
                        let n = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        let spec = caps.get(2).unwrap().as_str();
                        let key = (lhs.to_string(), spec.to_string());
                        *storage.entry(key).or_insert(0) += n;
                    } else {
                        panic!("No match for `{}`", item);
                    }
                }
            }
        });

    // ===========================================

    let mut graph = DiGraphMap::<&str, usize>::new();
    for (key, value) in storage.iter() {
        // println!("{:?}: {}", key, value);
        let (from, to) = key;
        if let Some(w) = graph.edge_weight_mut(from, to) {
            *w += value;
        } else {
            graph.add_edge(from, to, *value);
        }
    }
    // println!("Graph: {:?}", graph);
    // println!("{}", Dot::new(&graph));

    assert!(!petgraph::algo::is_cyclic_directed(&graph));

    // ===========================================

    let reversed = Reversed(&graph);
    let mut bfs_backward = Bfs::new(&reversed, "shiny gold");
    let mut total_backward = 0;
    while let Some(v) = bfs_backward.next(&reversed) {
        total_backward += 1;
    }

    // ===========================================

    let mut visited = HashSet::new();
    let mut bfs = Bfs::new(&graph, "shiny gold");
    while let Some(v) = bfs.next(&graph) {
        visited.insert(v);
    }
    let topo: Vec<&str> = petgraph::algo::toposort(&graph, None).unwrap();
    let mut weights = HashMap::new();
    println!("Topological sort (reversed):");
    for v in rev(topo) {
        if !visited.contains(v) {
            // println!(" - skipping {}", v);
            continue;
        }
        let w = 1 + graph
            .neighbors(v)
            .map(|n| graph.edge_weight(v, n).unwrap() * weights[n])
            .sum::<usize>();
        println!(" - node `{}`: weight = {}", v, w);
        weights.insert(v, w);
    }
    let total_forward = weights["shiny gold"];

    // ===========================================

    println!(
        "Total backward-BFS count (excluding start): {}",
        total_backward - 1
    );
    println!(
        "Total rev-toposort sum (excluding start): {}",
        total_forward - 1
    );

    Ok(())
}
