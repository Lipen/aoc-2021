use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
use std::ops::Rem;
use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;
use num_integer::{ExtendedGcd, Integer};

#[derive(Debug)]
struct Schedule {
    ids: Vec<Option<u64>>,
}

impl FromStr for Schedule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ids = s.split(',').map(|p| p.parse::<u64>().ok()).collect_vec();
        Ok(Schedule { ids })
    }
}

trait SumRem<T = Self>
where
    T: Rem<Output = T>,
{
    fn sum_rem<I>(iter: I, modulo: T) -> Self
    where
        I: Iterator<Item = T>;
}

impl<T> SumRem for T
where
    T: Copy + Sum + Rem<Output = T>,
{
    fn sum_rem<I>(iter: I, modulo: T) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.map(|x| x % modulo).sum::<T>() % modulo
    }
}

trait IteratorExt: Iterator {
    fn sum_rem<S>(self, modulo: Self::Item) -> S
    where
        Self: Sized,
        S: SumRem<Self::Item>,
        Self::Item: Rem<Output = Self::Item>,
    {
        SumRem::sum_rem(self, modulo)
    }
}

impl<I> IteratorExt for I where I: Iterator {}

fn modulus_inverse(x: i64, y: i64) -> Option<u64> {
    // solution for `a*x + b*y = gcd(x,y)` is egcd = (gcd, a, b)
    let ExtendedGcd {
        gcd, x: a, y: _b, ..
    } = Integer::extended_gcd(&x, &y);

    if gcd == 1 {
        let m = mod_fix(a, y);
        println!(
            "egcd({}, {}) = ({}, {}, {}), mod-inv is {}",
            x, y, gcd, a, _b, m
        );
        Some(m as u64)
    } else {
        None
    }
}

fn mod_fix(x: i64, m: i64) -> u64 {
    ((x % m + m) % m) as u64
}

fn solve_part_one(schedule: &Schedule, start: i32) {
    let ids = schedule.ids.iter().filter_map(|&x| x).collect_vec();
    let (nearest_time, nearest_id) = ids
        .iter()
        .map(|&t| {
            (
                ((start as f32 / t as f32).ceil() * t as f32) as i32,
                t as i32,
            )
        })
        .min()
        .unwrap();
    let diff = nearest_time - start;
    println!(
        "Nearest bus is {} at {}, which is {} minutes away from the start ({})",
        nearest_id, nearest_time, diff, start
    );
    println!("product = {}", nearest_id * diff);
}

fn solve_part_two(schedule: &Schedule) {
    let data = schedule
        .ids
        .iter()
        .enumerate()
        .filter_map(|(i, id)| id.map(|a| (i, a)))
        .collect_vec();
    let modulii = data.iter().map(|&(_, x)| x).collect_vec();
    println!("modulii = {:?}", modulii);
    let product: u64 = modulii.iter().product();
    println!("product = {}", product);
    let residues = data
        .iter()
        .map(|&(i, x)| mod_fix(x as i64 - i as i64, x as i64))
        .collect_vec();
    println!("residues = {:?}", residues);
    let ms = modulii.iter().map(|&p| product / p).collect_vec();
    println!("ms = {:?}", ms);
    let t: u64 = (0..data.len())
        .map(|i| {
            let modulus = modulii[i];
            let residue = residues[i];
            let m = product / modulus;
            residue * m * modulus_inverse(m as i64, modulus as i64).unwrap()
        })
        .sum_rem(product);
    println!("t = {}", t);
}

fn main() -> Result<()> {
    let path = "data/input.txt";
    // let path = "data/sample.txt"; // 1068788
    // let path = "data/mini1.txt"; // 3417
    // let path = "data/mini2.txt"; // 754018
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();
    let start = lines.next().unwrap()?.parse::<i32>()?;
    let schedule = lines.next().unwrap()?.parse::<Schedule>()?;

    println!("Solving part 1...");
    solve_part_one(&schedule, start);
    println!();
    println!("Solving part 2...");
    solve_part_two(&schedule);

    Ok(())
}
