use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{AddAssign, Mul};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use once_cell_regex::regex;

#[derive(Debug)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"^([NSEWLRF])(\d+)$");
        if let Some(caps) = re.captures(s) {
            let value = caps.get(2).unwrap().as_str().parse::<i32>()?;
            let action = match caps.get(1).unwrap().as_str() {
                "N" => Action::North(value),
                "S" => Action::South(value),
                "E" => Action::East(value),
                "W" => Action::West(value),
                "L" => Action::Left(value),
                "R" => Action::Right(value),
                "F" => Action::Forward(value),
                _ => return Err(anyhow!("Bad action")),
            };
            Ok(action)
        } else {
            Err(anyhow!("No match"))
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position(i32, i32);

impl Position {
    fn add(&mut self, d: (i32, i32)) {
        self.0 += d.0;
        self.1 += d.1;
    }
}

impl AddAssign<(i32, i32)> for Position {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl AddAssign<Waypoint> for Position {
    fn add_assign(&mut self, rhs: Waypoint) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug, Copy, Clone)]
struct Waypoint(i32, i32);

impl Waypoint {
    fn rotate_left(&mut self, angle: i32) {
        *self = match angle {
            90 => self.left(),
            180 => self.opposite(),
            270 => self.right(),
            _ => panic!("Bad angle {}", angle),
        };
    }

    fn rotate_right(&mut self, angle: i32) {
        *self = match angle {
            90 => self.right(),
            180 => self.opposite(),
            270 => self.left(),
            _ => panic!("Bad angle {}", angle),
        };
    }

    #[must_use]
    fn left(&self) -> Waypoint {
        Waypoint(-self.1, self.0)
    }

    #[must_use]
    fn right(&self) -> Waypoint {
        Waypoint(self.1, -self.0)
    }

    #[must_use]
    fn opposite(&self) -> Waypoint {
        Waypoint(-self.0, -self.1)
    }
}

impl AddAssign<(i32, i32)> for Waypoint {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Mul<i32> for Waypoint {
    type Output = Waypoint;

    fn mul(self, rhs: i32) -> Self::Output {
        Waypoint(rhs * self.0, rhs * self.1)
    }
}

#[derive(Debug)]
struct State {
    position: Position,
    waypoint: Waypoint,
}

impl State {
    //noinspection DuplicatedCode
    fn eval1(&self, action: &Action) -> State {
        let State {
            mut position,
            mut waypoint,
        } = *self;

        match *action {
            Action::North(value) => position += (0, value),
            Action::South(value) => position += (0, -value),
            Action::East(value) => position += (value, 0),
            Action::West(value) => position += (-value, 0),
            Action::Left(angle) => waypoint.rotate_left(angle),
            Action::Right(angle) => waypoint.rotate_right(angle),
            Action::Forward(times) => position += waypoint * times,
        }

        State { position, waypoint }
    }

    //noinspection DuplicatedCode
    fn eval2(&self, action: &Action) -> State {
        let State {
            mut position,
            mut waypoint,
        } = *self;

        match *action {
            Action::North(value) => waypoint += (0, value),
            Action::South(value) => waypoint += (0, -value),
            Action::East(value) => waypoint += (value, 0),
            Action::West(value) => waypoint += (-value, 0),
            Action::Left(angle) => waypoint.rotate_left(angle),
            Action::Right(angle) => waypoint.rotate_right(angle),
            Action::Forward(times) => position += waypoint * times,
        }

        State { position, waypoint }
    }
}

fn solve_part_one(data: &[Action]) {
    println!("Solving part 1...");
    let mut state = State {
        position: Position(0, 0),
        waypoint: Waypoint(1, 0),
    };

    for action in data {
        state = state.eval1(action);
    }

    println!("[part1] Done with {:?}", state);
    println!(
        "[part1] Manhattan distance from the start: {}",
        state.position.0.abs() + state.position.1.abs()
    )
}

fn solve_part_two(data: &[Action]) {
    println!("Solving part 2...");
    let mut state = State {
        position: Position(0, 0),
        waypoint: Waypoint(10, 1),
    };

    for action in data {
        state = state.eval2(action);
    }

    println!("[part2] Done with {:?}", state);
    println!(
        "[part2] Manhattan distance from the start: {}",
        state.position.0.abs() + state.position.1.abs()
    )
}

fn main() -> Result<()> {
    let path = "data/input.txt";
    // let path = "data/sample.txt";
    let data: Vec<_> = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.parse::<Action>())
        // .collect_vec();
        .try_collect()?;
    // println!("[debug] {:?}", data);

    solve_part_one(&data);
    println!();
    solve_part_two(&data);

    Ok(())
}
