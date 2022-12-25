use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::{rev, Either, Itertools};

fn check_first(x: u64, data: &[u64]) -> bool {
    // println!("Checking {} in {:?}", x, data);
    data.iter().combinations(2).any(|comb| {
        let a = comb[0];
        let b = comb[1];
        a != b && a + b == x
    })
}

fn find_invalid_number_first(data: &[u64]) -> (usize, u64) {
    for i in 25..data.len() {
        let x = data[i];
        if !check_first(x, &data[i - 25..i]) {
            return (i, x);
        }
    }
    panic!("Not found")
}

fn main() -> Result<()> {
    let path = "data/input.txt";
    let data = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();

    let (index, value) = find_invalid_number_first(&data);
    println!("Found invalid number: {}-th = {}", index, value);

    for i in 0..data.len() {
        let mut j = i + 1;
        let mut s = data[i] + data[j];
        while s < value {
            j += 1;
            s += data[j];
        }
        if s == value {
            let range = &data[i..=j];
            let min = range.iter().min().unwrap();
            let max = range.iter().max().unwrap();
            println!(
                "Found sum-range for {} of length {} from {}-th ({}) to {}-th ({})",
                value,
                j - i + 1,
                i,
                data[i],
                j,
                data[j]
            );
            println!("Sum of min/max = {}+{} = {}", min, max, min + max);
            println!("Range: {:?}", range);
            break;
        }
    }

    Ok(())
}
