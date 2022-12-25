use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::Location::{Empty, Occupied};

struct Line {
    data: Vec<Location>,
}

#[derive(PartialEq)]
enum Location {
    Empty,
    Occupied,
}

impl Location {
    fn new(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Empty),
            '#' => Ok(Occupied),
            _ => Err(anyhow!("Bad location character: `{}`", c)),
        }
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.chars().map(|c| Location::new(c)).try_collect()?;
        Ok(Line { data })
    }
}

fn main() -> Result<()> {
    let path = "data/input.txt";
    let lines: Vec<Line> = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.parse().unwrap())
        .collect();

    // Note:
    //  i -- vertical
    //  j -- horizontal

    let steps = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: u64 = steps
        .iter()
        .map(|&(step_j, step_i)| {
            let mut counter = 0;
            let mut j = 0;
            for i in (0..lines.len()).step_by(step_i).progress() {
                let ref line = lines[i].data;
                let ref loc = line[j % line.len()];
                if *loc == Occupied {
                    counter += 1;
                }
                j += step_j;
                // std::thread::sleep(std::time::Duration::from_millis(1))
            }
            println!(
                "Count trees for slope ({}, {}): {}",
                step_j, step_i, counter
            );
            counter as u64
        })
        .product();
    println!("Product: {}", product);

    Ok(())
}
