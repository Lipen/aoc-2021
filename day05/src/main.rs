use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let path = "data/input.txt";
    let data: Vec<_> = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| {
            assert_eq!(line.len(), 10);
            let mut chars = line.chars();
            let row: i32 = (0..7)
                .rev()
                .map(|i| match chars.next().unwrap() {
                    'B' => 1 << i,
                    'F' => 0,
                    _ => panic!("bad char"),
                })
                .sum();
            let col: i32 = (0..3)
                .rev()
                .map(|j| match chars.next().unwrap() {
                    'R' => 1 << j,
                    'L' => 0,
                    _ => panic!("bad char"),
                })
                .sum();
            (row, col)
        })
        .collect();
    let ids: Vec<_> = data.iter().map(|(row, col)| row * 8 + col).collect();
    let min_id = ids.iter().min().unwrap();
    let max_id = ids.iter().max().unwrap();
    println!("Minimum ID: {}", min_id);
    println!("Maximum ID: {}", max_id);

    for x in *min_id..*max_id {
        if !ids.contains(&x) {
            println!("Missing ID: {}", x);
            break;
        }
    }

    Ok(())
}
