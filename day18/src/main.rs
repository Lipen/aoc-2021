use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use anyhow::Result;
use itertools::Itertools;

mod parser_rev;
mod parser_same;

fn solve_part_one(data: &[String]) -> i64 {
    let parsed = data
        .iter()
        .map(|line| parser_same::parse(line))
        .collect_vec();
    // println!("Parsed values: {:?}", parsed);
    let sum = parsed.iter().sum();
    println!("Sum: {}", sum);
    sum
}

fn solve_part_two(data: &[String]) -> i64 {
    let parsed = data
        .iter()
        .map(|line| parser_rev::parse(line))
        .collect_vec();
    // println!("Parsed values: {:?}", parsed);
    let sum = parsed.iter().sum();
    println!("Sum: {}", sum);
    sum
}

fn read_data(path: &str) -> Result<Vec<String>> {
    let data = BufReader::new(File::open(path)?).lines().try_collect()?;
    Ok(data)
}

fn main() -> Result<()> {
    let start_time = Instant::now();
    let path = "data/input.txt";
    // let path = "data/sample.txt"; // 71
    let data = read_data(path)?;

    println!("Solving part 1...");
    solve_part_one(&data);
    println!();

    println!("Solving part 2...");
    solve_part_two(&data);
    println!();

    println!("All done in {:.3} s", start_time.elapsed().as_secs_f32());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/input.txt";
        let data = read_data(path).unwrap();
        let result = solve_part_one(&data);
        assert_eq!(98621258158412, result);
    }

    #[test]
    fn test_part_two() {
        let path = "data/input.txt";
        let data = read_data(path).unwrap();
        let result = solve_part_two(&data);
        assert_eq!(241216538527890, result);
    }
}
