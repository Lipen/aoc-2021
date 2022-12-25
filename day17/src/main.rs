#[macro_use]
extern crate derive_new;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;
use std::ops::Add;
use std::time::Instant;

use anyhow::Result;
use itertools::{sorted, Itertools};
use once_cell::sync::Lazy;
use pipe_trait::Pipe;

// Axes:
//   |
// --+---> (y)
//   |
//   v (x)

#[derive(Debug, Copy, Clone)]
enum State {
    Empty,
    Active,
}

impl From<char> for State {
    fn from(c: char) -> Self {
        use State::*;
        match c {
            '.' => Empty,
            '#' => Active,
            _ => panic!("Bad char `{}`", c),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, new)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point {
    fn neighbors(&self) -> impl Iterator<Item = Point> + '_ {
        DS.iter().map(move |&d| self + d)
    }
}

impl Add<(i32, i32, i32, i32)> for &Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32, i32, i32)) -> Self::Output {
        Point {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
            w: self.w + rhs.3,
        }
    }
}

static DS: Lazy<Vec<(i32, i32, i32, i32)>> = Lazy::new(|| {
    // let ds = itertools::iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
    //     .filter(|&(x,y,z,w)| !(x == 0 && y == 0 && z == 0 && w == 0))
    //     .collect_vec();
    let mut ds = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if !(x == 0 && y == 0 && z == 0 && w == 0) {
                        ds.push((x, y, z, w));
                    }
                }
            }
        }
    }
    assert_eq!(80, ds.len());
    ds
});

fn conway_cubes_step(data: &HashSet<Point>) -> HashSet<Point> {
    let is_active = |p: &Point| -> bool { data.contains(p) };
    data.iter()
        // Note: `Point::neighbors` returns an iterator over neighbors,
        //  not including the point itself. Here, we need to also consider
        //  the point `p` itself, so we `chain` it to the resulting iterator.
        .flat_map(|p| p.neighbors().chain(once(p.clone())))
        .filter(|p| {
            let active_neighbors = p.neighbors().filter(is_active).count();
            // println!("{:?} has {} active neighbor(s)", p, active_neighbors);
            if is_active(p) {
                active_neighbors == 2 || active_neighbors == 3
            } else {
                active_neighbors == 3
            }
        })
        .collect()
}

fn print_active(data: &HashSet<Point>) {
    use itertools::MinMaxResult::{MinMax, NoElements, OneElement};
    let (x_min, x_max) = {
        let x_minmax = data.iter().map(|p| p.x).minmax();
        match x_minmax {
            NoElements => panic!("No xs"),
            OneElement(t) => (t, t),
            MinMax(min, max) => (min, max),
        }
    };
    let (y_min, y_max) = {
        let y_minmax = data.iter().map(|p| p.y).minmax();
        match y_minmax {
            NoElements => panic!("No ys"),
            OneElement(t) => (t, t),
            MinMax(min, max) => (min, max),
        }
    };

    for z in data.iter().map(|p| p.z).unique().pipe(sorted) {
        for w in data.iter().map(|p| p.w).unique().pipe(sorted) {
            println!(
                "\nz = {}, w = {}  (x: [{}..{}], y: [{}..{}])",
                z, w, x_min, x_max, y_min, y_max
            );
            for x in x_min..=x_max {
                let s = (y_min..=y_max)
                    .map(|y| {
                        if data.contains(&Point::new(x, y, z, w)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>();
                println!("{}", s);
            }
        }
    }
}

fn solve(data: &[Vec<State>]) -> usize {
    let mut active = HashSet::<Point>::new();
    for (x, row) in data.iter().enumerate() {
        for (y, state) in row.iter().enumerate() {
            if matches!(state, State::Active) {
                active.insert(Point::new(x as i32, y as i32, 0, 0));
            }
        }
    }
    println!("Initial active cubes:");
    print_active(&active);

    let n = 6;
    for i in 0..n {
        println!("Cycle {}...", i + 1);
        active = conway_cubes_step(&active);
        // println!("Active cubes after {} cycle(s)):", i + 1);
        // print_active(&active);
    }

    println!();
    println!(
        "Number of active cubes after {} cycles: {}",
        n,
        active.len()
    );
    active.len()
}

fn main() -> Result<()> {
    let start_time = Instant::now();
    // let path = "data/input.txt";
    let path = "data/sample.txt"; // 848, was 112 for 3-dim problem (part one)
    let data = BufReader::new(File::open(path)?)
        .lines()
        .map(|r| r.expect("Could not read line"))
        .map(|line| line.chars().map(|c| State::from(c)).collect_vec())
        .collect_vec();

    println!("Solving...");
    solve(&data);

    println!("All done in {:.2} s", start_time.elapsed().as_secs_f32());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let path = "data/sample.txt";
        let data = BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|r| r.expect("Could not read line"))
            .map(|line| line.chars().map(|c| State::from(c)).collect_vec())
            .collect_vec();
        let result = solve(&data);
        assert_eq!(result, 848);
    }
}
