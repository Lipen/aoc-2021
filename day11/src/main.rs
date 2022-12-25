use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::{fmt, iter, ops};

use anyhow::{anyhow, Error, Result};
use itertools::{any, rev, Itertools};

static DF: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, PartialEq)]
enum Seat {
    /// '.' is a floor.
    Floor,
    /// 'L' is an empty seat.
    Empty,
    /// '#' is an occupied seat.
    Occupied,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Seat::Floor => '.',
                Seat::Empty => 'L',
                Seat::Occupied => '#',
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Grid<T> {
    data: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    fn new_with<F>(height: usize, width: usize, f: F) -> Self
    where
        F: Fn(usize, usize) -> T,
    {
        let data = (0..height)
            .map(|i| (0..width).map(|j| f(i, j)).collect_vec())
            .collect_vec();
        Grid {
            data,
            height,
            width,
        }
    }

    fn get(&self, i: usize, j: usize) -> Option<&T> {
        if let Some(row) = self.data.get(i) {
            row.get(j)
        } else {
            None
        }
    }

    fn neighbors_adjacent(&self, i: usize, j: usize) -> impl Iterator<Item = Option<&T>> {
        DF.iter().map(move |(dx, dy)| {
            let a = (i as i32 + dx) as usize;
            let b = (j as i32 + dy) as usize;
            self.get(a, b)
        })
    }

    fn neighbors_in_direction(
        &self,
        mut i: usize,
        mut j: usize,
        d: (i32, i32),
    ) -> impl Iterator<Item = &T> {
        let (dx, dy) = d;
        iter::from_fn(move || {
            i = (i as i32 + dx) as usize;
            j = (j as i32 + dy) as usize;
            self.get(i, j)
        })
    }
}

impl Grid<Seat> {
    fn first_nonfloor_neighbor_in_direction(
        &self,
        i: usize,
        j: usize,
        d: (i32, i32),
    ) -> Option<&Seat> {
        self.neighbors_in_direction(i, j, d)
            .find(|x| !matches!(x, Seat::Floor))
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = data[0].len();
        Grid {
            data,
            height,
            width,
        }
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.iter() {
            let s = row.iter().map(|x| format!("{}", x)).collect::<String>();
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl<T, Idx> ops::Index<Idx> for Grid<T>
where
    Idx: Into<(usize, usize)>,
{
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        let (i, j) = index.into();
        &self.data[i][j]
    }
}

fn round_part_one(grid: &Grid<Seat>) -> Grid<Seat> {
    Grid::new_with(grid.height, grid.width, |i, j| {
        match grid.data[i][j] {
            Seat::Floor => Seat::Floor,
            Seat::Empty => {
                // If a seat is empty (L) and there are no occupied seats adjacent to it,
                //  the seat becomes occupied.
                let some_neighbor_occupied = grid
                    .neighbors_adjacent(i, j)
                    .any(|x| matches!(x, Some(Seat::Occupied)));

                if some_neighbor_occupied {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
            Seat::Occupied => {
                // If a seat is occupied (#) and four or more seats adjacent to it
                //  are also occupied, the seat becomes empty.
                let occupied_neighbors = grid
                    .neighbors_adjacent(i, j)
                    .filter(|x| matches!(x, Some(Seat::Occupied)))
                    .count();

                if occupied_neighbors >= 4 {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
        }
    })
}

fn round_part_two(grid: &Grid<Seat>) -> Grid<Seat> {
    Grid::new_with(grid.height, grid.width, |i, j| {
        match grid.data[i][j] {
            Seat::Floor => Seat::Floor,
            Seat::Empty => {
                // empty seats that see no occupied seats become occupied
                let see_occupied_neighbor = DF.iter().any(|&d| {
                    let first_nonfloor_neighbor =
                        grid.first_nonfloor_neighbor_in_direction(i, j, d);
                    matches!(first_nonfloor_neighbor, Some(Seat::Occupied))
                });

                if see_occupied_neighbor {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
            Seat::Occupied => {
                // it now takes five or more visible occupied seats for an occupied seat to become empty
                let occupied_visible_neighbors = DF
                    .iter()
                    .filter_map(|&d| grid.first_nonfloor_neighbor_in_direction(i, j, d))
                    .filter(|x| matches!(x, Seat::Occupied))
                    .count();

                if occupied_visible_neighbors >= 5 {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
        }
    })
}

struct Solution {
    steady_state_round: usize,
    occupied_seats: usize,
}

fn solve<F>(grid: &Grid<Seat>, round: F) -> Solution
where
    F: Fn(&Grid<Seat>) -> Grid<Seat>,
{
    let mut grid = grid.clone();
    // println!("Initial grid:\n{}", grid);
    let mut iteration = 0;

    loop {
        // let occ_grid = Grid::new_with(grid.height, grid.width, |i,j| {
        //     DF
        //         .iter()
        //         .filter_map(|&d| grid.first_nonfloor_neighbor_in_direction(i, j, d))
        //         .filter(|x| matches!(x, Seat::Occupied))
        //         .count()
        // });
        // println!("Occupied visible neighbors on round {}:\n{}",iteration, occ_grid);

        iteration += 1;
        let new_grid = round(&grid);
        // println!("Grid after round {}:\n{}", iteration, new_grid);

        let is_stabilized = new_grid
            .data
            .iter()
            .enumerate()
            .all(|(i, row)| row.iter().enumerate().all(|(j, x)| *x == grid.data[i][j]));

        if is_stabilized {
            println!("Steady state after round {}", iteration);
            let occupied = new_grid
                .data
                .iter()
                .flatten()
                .filter(|x| matches!(x, Seat::Occupied))
                .count();
            println!("Total occupied seats: {}", occupied);

            return Solution {
                steady_state_round: iteration,
                occupied_seats: occupied,
            };
        }

        grid = new_grid;
    }
}

fn main() -> Result<()> {
    let path = "data/input.txt";
    // let path = "data/sample.txt";
    let grid: Grid<Seat> = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Seat::Floor,
                    'L' => Seat::Empty,
                    // '#' => Seat::Occupied,
                    _ => panic!("Bad char `{}`", c),
                })
                .collect_vec()
        })
        .collect_vec()
        .into();

    // println!("[debug] {:?}", grid);

    println!("Solving part 1...");
    let solution_part_one = solve(&grid, round_part_one);
    println!();
    println!("Solving part 2...");
    let solution_part_two = solve(&grid, round_part_two);

    println!();
    println!("Solutions:");
    for (i, s) in [solution_part_one, solution_part_two].iter().enumerate() {
        println!(
            "[part{}] Steady state after round {} with {} occupied seats",
            i + 1,
            s.steady_state_round,
            s.occupied_seats
        );
    }

    Ok(())
}
