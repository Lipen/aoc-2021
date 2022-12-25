use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use anyhow::Result;
use itertools::Itertools;
use once_cell_regex::regex;
use tap::{Pipe, Tap};

use crate::side::{Direction, Side};
use crate::tile::Tile;

mod side;
mod tile;

fn solve_part_one(tiles: &[Tile]) -> usize {
    let n = tiles.len(); // total number of tiles
    let k = tiles[0].data.len(); // tile size (each tile is a square)
    let sides = {
        use Side::*;
        [Top, Bot, Left, Right]
    };
    let mut compatible = HashSet::new();
    let mut compatible_flipped = HashSet::new();

    for i in 0..n {
        let tile1 = &tiles[i];
        for side1 in sides.iter().copied() {
            let s1 = tile1.data_on_side(side1, Direction::Inner);
            for j in (i + 1)..n {
                let tile2 = &tiles[j];
                for side2 in sides.iter().copied() {
                    let s2 = tile2.data_on_side(side2, Direction::Outer);
                    if s1 == s2 {
                        compatible.insert(((tile1.id, side1), (tile2.id, side2)));
                    }
                    let s2_rev = tile2.data_on_side(side2, Direction::Inner);
                    if s1 == s2_rev {
                        compatible_flipped.insert(((tile1.id, side1), (tile2.id, side2)));
                    }
                }
            }
        }
    }

    // println!("Compatible:");
    // for item in compatible.iter() {
    //     println!(" - {:?}", item);
    // }
    // println!("Compatible (flipped):");
    // for item in compatible_flipped.iter() {
    //     println!(" - {:?}", item);
    // }

    let mut p = 1;
    for tile in tiles.iter() {
        let comp = tiles
            .iter()
            .filter(|t| {
                compatible.iter().any(|&((a, _), (b, _))| {
                    (a == tile.id && b == t.id) || (b == tile.id && a == t.id)
                })
            })
            .collect_vec();
        let comp_rev = tiles
            .iter()
            .filter(|t| {
                compatible_flipped.iter().any(|&((a, _), (b, _))| {
                    (a == tile.id && b == t.id) || (b == tile.id && a == t.id)
                })
            })
            .collect_vec();
        if comp.len() + comp_rev.len() != 2 {
            continue;
        }
        p *= tile.id;
        println!(
            "Compatible with tile {}: {:?} ++ {:?} == {} + {} = {}",
            tile.id,
            comp.iter().map(|t| t.id).collect_vec(),
            comp_rev.iter().map(|t| t.id).collect_vec(),
            comp.len(),
            comp_rev.len(),
            comp.len() + comp_rev.len()
        );
    }
    println!("product of corners: {}", p);

    42
}

fn read_data(path: &str) -> Result<Vec<Tile>> {
    let mut lines = BufReader::new(File::open(path)?)
        .lines()
        .map(|r| r.expect("Could not read line"));
    let mut tiles = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let re = regex!(r"Tile (\d+):");
        let id = re
            .captures(&line)
            .ok_or(anyhow::anyhow!("No match for `{}`", line))?
            .pipe(|caps| caps.get(1).unwrap().as_str().parse::<usize>().unwrap());
        let mut data = Vec::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let row = line.chars().collect();
            data.push(row);
        }

        let tile = Tile::new(id, data);
        tiles.push(tile);
    }

    Ok(tiles)
}

fn main() -> Result<()> {
    let start_time = Instant::now();
    let path = "data/input.txt";
    // let path = "data/sample.txt"; // 20899048083289
    let tiles = read_data(path)?;

    println!("tiles (length = {}):", tiles.len());
    for tile in tiles.iter() {
        println!("{}", tile);
    }

    println!("Solving part one...");
    solve_part_one(&tiles);
    // println!();
    // println!("Solving part two...");
    // solve_part_two(&rules, &messages);

    println!("All done in {:.2} s", start_time.elapsed().as_secs_f32());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        // 1951    2311    3079
        // 2729    1427    2473
        // 2971    1489    1171
        let path = "data/sample.txt";
        let tiles = read_data(path).unwrap();
        let result = solve_part_one(&tiles);
        // 1951 * 3079 * 2971 * 1171 = 20899048083289
        assert_eq!(result, 20899048083289);
    }
}
