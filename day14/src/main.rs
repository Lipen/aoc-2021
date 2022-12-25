use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use indicatif::ProgressIterator;
use itertools::Itertools;
use once_cell_regex::regex;

#[derive(Debug)]
enum Instruction {
    Mask(Vec<Bit>),
    Mem { address: usize, value: u64 },
}

#[derive(Debug, Copy, Clone)]
enum Bit {
    Zero,
    One,
    Floating,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_mask = regex!(r"^mask = ([X01]{36})$");
        let re_mem = regex!(r"^mem\[(\d+)] = (\d+)$");
        if let Some(caps) = re_mask.captures(s) {
            let mask = caps
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .map(|c| c.try_into())
                .try_collect()?;
            Ok(Instruction::Mask(mask))
        } else if let Some(caps) = re_mem.captures(s) {
            let address = caps.get(1).unwrap().as_str().parse::<usize>()?;
            let value = caps.get(2).unwrap().as_str().parse::<u64>()?;
            Ok(Instruction::Mem { address, value })
        } else {
            Err(anyhow!("No match for `{}`", s))
        }
    }
}

impl TryFrom<char> for Bit {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Bit::Zero),
            '1' => Ok(Bit::One),
            'X' => Ok(Bit::Floating),
            _ => Err(anyhow!("Bad char `{}` in mask", c)),
        }
    }
}

impl From<bool> for Bit {
    fn from(b: bool) -> Self {
        if b {
            Bit::One
        } else {
            Bit::Zero
        }
    }
}

fn apply_mask_to_value(mask: &Vec<Bit>, mut value: u64) -> u64 {
    for (i, m) in mask.iter().rev().enumerate() {
        match m {
            Bit::One => value |= 1 << i,
            Bit::Zero => value &= !(1 << i),
            Bit::Floating => { /* Do nothing */ }
        }
    }
    value
}

fn solve_part_one(data: &[Instruction]) {
    let mut memory = HashMap::<usize, u64>::new();
    let mut global_mask = vec![Bit::Floating; 36];

    for instruction in data.iter() {
        match instruction {
            Instruction::Mask(mask) => {
                global_mask = mask.clone();
            }
            &Instruction::Mem { address, value } => {
                memory.insert(address, apply_mask_to_value(&global_mask, value));
            }
        }
    }

    let sum: u64 = memory.values().sum();
    println!("sum = {}", sum);
}

fn get_bit(value: u64, n: usize) -> Bit {
    Bit::from(value & (1 << n) > 0)
}

fn bits_to_u64(bits: &[Bit]) -> u64 {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, b)| match b {
            Bit::Zero => 0,
            Bit::One => 1 << i,
            _ => panic!("Bad bit `{:?}`", b),
        })
        .sum()
}

fn solve_part_two(data: &[Instruction]) {
    let mut memory = HashMap::<usize, u64>::new();
    let mut global_mask = vec![Bit::Zero; 36];

    for instruction in data.iter() {
        println!("instruction: {:?}", instruction);
        match instruction {
            Instruction::Mask(mask) => {
                global_mask = mask.clone();
            }
            &Instruction::Mem { address, value } => {
                assert!(address < 1 << 36, "Address is larger than 36 bits");

                let mask = &global_mask;

                let floating_bits = mask
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| matches!(b, Bit::Floating))
                    .map(|(i, _)| i)
                    .collect_vec();
                // println!("Floating bits: {:?}", floating_bits);

                let n = mask.len();
                let masked = mask
                    .iter()
                    .enumerate()
                    .map(|(i, b)| match b {
                        Bit::Zero => get_bit(address as u64, n - i - 1),
                        &b => b,
                    })
                    .collect_vec();

                // println!("address= {}", (0..36).rev().map(|i| get_bit(address as u64, i)).map(|b| match b {
                //     Bit::One => '1',
                //     Bit::Zero => '0',
                //     Bit::Floating => 'x',
                // }).collect::<String>());
                // println!("mask   = {}", mask.iter().map(|b| match b {
                //     Bit::One => '1',
                //     Bit::Zero => '0',
                //     Bit::Floating => 'x',
                // }).collect::<String>());
                // println!("masked = {}", masked.iter().map(|b| match b {
                //     Bit::One => '1',
                //     Bit::Zero => '0',
                //     Bit::Floating => 'x',
                // }).collect::<String>());

                floating_bits
                    .iter()
                    .powerset()
                    .map(|subset| {
                        masked
                            .iter()
                            .enumerate()
                            .map(|(i, b)| match b {
                                Bit::Floating => Bit::from(subset.contains(&&i)),
                                &b => b,
                            })
                            .collect_vec()
                    })
                    .progress_count(2u64.pow(floating_bits.len() as u32))
                    .for_each(|bits| {
                        let addr = bits_to_u64(&bits) as usize;
                        memory.insert(addr, value);
                    })
            }
        }
    }

    let sum: u64 = memory.values().sum();
    println!("sum = {}", sum);
}

fn main() -> Result<()> {
    // let path = "data/input.txt";
    // let path = "data/sample.txt"; // 165
    let path = "data/sample2.txt"; // 208
    let data: Vec<_> = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.parse::<Instruction>())
        .try_collect()?;
    // println!("[debug] {:?}", data);

    println!("Solving part 1...");
    solve_part_one(&data);
    println!();
    println!("Solving part 2...");
    solve_part_two(&data);

    Ok(())
}
