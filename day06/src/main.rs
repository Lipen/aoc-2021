use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Result};

#[derive(Debug, Default)]
struct Item {
    union: HashSet<char>,
    intersection: HashSet<char>,
}

fn main() -> Result<()> {
    let path = "data/input.txt";
    let reader = BufReader::new(File::open(path)?);

    let mut data: Vec<Item> = Vec::new();
    let mut temp: Option<Item> = None;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if let Some(item) = temp {
                data.push(item);
                temp = None
            }
        } else {
            let chars = line.chars().collect();
            if let Some(ref mut item) = temp {
                // Update union
                item.union = item.union.union(&chars).copied().collect();
                // Update intersection
                item.intersection = item.intersection.intersection(&chars).copied().collect();
            } else {
                // Create new item using current chars
                // Note: only one `.clone()` is necessary here for `chars`,
                //  another one can `move`, but I do clone both just for consistency.
                temp = Some(Item {
                    union: chars.clone(),
                    intersection: chars.clone(),
                });
            }
        }
    }

    // Note: do not forget to push the last one!
    if let Some(item) = temp {
        data.push(item);
        temp = None
    }

    // data.iter().for_each(|x| {
    //     println!("{:?}", x);
    // });

    let total_union: usize = data.iter().map(|x| x.union.len()).sum();
    let total_intersection: usize = data.iter().map(|x| x.intersection.len()).sum();
    println!("Total union sum: {}", total_union);
    println!("Total intersection sum: {}", total_intersection);

    Ok(())
}
