use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let path = "data/input.txt";
    let data: Vec<_> = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    println!("Solving part one...");
    'outer1: for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            let a = data[i];
            let b = data[j];
            if a + b == 2020 {
                println!("Found!");
                println!("{} + {} = 2020", a, b);
                println!("{} * {} = {}", a, b, a * b);
                break 'outer1;
            }
        }
    }

    println!("===================");
    println!("Solving part two...");
    'outer2: for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            for k in (j + 1)..data.len() {
                let a = data[i];
                let b = data[j];
                let c = data[k];
                if a + b + c == 2020 {
                    println!("Found!");
                    println!("{} + {} + {} = 2020", a, b, c);
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    break 'outer2;
                }
            }
        }
    }

    Ok(())
}
