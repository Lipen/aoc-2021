use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Item {
    policy: Policy,
    password: String,
}

#[derive(Debug)]
struct Policy {
    letter: char,
    min: usize,
    max: usize,
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s+(\w):\s+(\w+)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let letter = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let password = caps.get(4).unwrap().as_str().to_string();
        Ok(Item {
            policy: Policy { letter, min, max },
            password,
        })
    }
}

impl Item {
    fn is_valid_first(&self) -> bool {
        let Policy { letter, min, max } = self.policy;
        let count = self.password.chars().filter(|&c| c == letter).count();
        min <= count && count <= max
    }

    fn is_valid_second(&self) -> bool {
        let Policy {
            letter,
            min: i,
            max: j,
        } = self.policy;
        let len = self.password.len();
        if i > len || j > len {
            return false;
        }
        let a = self.password.chars().nth(i - 1).unwrap();
        let b = self.password.chars().nth(j - 1).unwrap();
        (a == letter) ^ (b == letter)
    }
}

fn main() -> Result<()> {
    let path = "data/input.txt";
    let items: Vec<Item> = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.parse().unwrap())
        .collect();

    let valid_first = items.iter().filter(|&x| Item::is_valid_first(x)).count();
    println!("Total valid items (first way): {}", valid_first);
    assert_eq!(620, valid_first);

    let valid_second = items.iter().filter(|&x| x.is_valid_second()).count();
    println!("Total valid items (second way): {}", valid_second);
    assert_eq!(727, valid_second);

    Ok(())
}
