use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::{rev, Either, Itertools};
use once_cell_regex::regex;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = regex!(r"^(acc|jmp|nop) ([+-]\d+)$")
            .captures(s)
            .ok_or_else(|| anyhow!("line does not match"))?;

        use Instruction::*;
        let arg = caps.get(2).unwrap().as_str().parse()?;
        Ok(match caps.get(1).unwrap().as_str() {
            "acc" => Acc(arg),
            "jmp" => Jmp(arg),
            "nop" => Nop(arg),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Default)]
struct State {
    pointer: usize,
    accumulator: i32,
}

impl State {
    fn new(pointer: usize, accumulator: i32) -> Self {
        State {
            pointer,
            accumulator,
        }
    }

    fn eval(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::Acc(arg) => {
                self.pointer += 1;
                self.accumulator += arg;
            }
            Instruction::Jmp(arg) => {
                self.pointer = (self.pointer as i32 + arg) as usize;
            }
            Instruction::Nop(_) => {
                self.pointer += 1;
            }
        }
    }
}

fn find_infinite_loop(program: &[Instruction]) -> Either<Box<State>, Box<State>> {
    let mut state = State::default();
    let mut visited = vec![false; program.len()];

    // println!("Executing a program...");
    loop {
        visited[state.pointer] = true;
        let instruction = &program[state.pointer];
        // println!(" - {:?} ::: {:?}", state, instruction);
        state.eval(instruction);

        if state.pointer >= program.len() {
            // No infinite loop found.
            return Either::Right(Box::new(state));
        }

        if visited[state.pointer] {
            // Found an infinite loop.
            return Either::Left(Box::new(state));
        }
    }
}

fn main() -> Result<()> {
    let path = "data/input.txt";
    let program = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect_vec();

    // println!("Program:");
    // for instruction in program.iter() {
    //     println!(" - {:?}", instruction);
    // }

    println!(">>> Searching for infinite loop in the original program...");
    match find_infinite_loop(&program) {
        Either::Left(state) => {
            println!("  - Infinite loop found!");
            println!("  - Last state: {:?}", state);
        }
        Either::Right(state) => {
            println!("  - Program ended without going into an infinite loop!");
            println!("  - Last state: {:?}", state);
            panic!("The given program has an infinite loop, but we could not detect it.");
        }
    }

    println!(">>> Trying to mutate program to make it terminate...");
    for i in 0..program.len() {
        use Instruction::*;
        if let Some(mutated_instruction) = match &program[i] {
            Jmp(arg) => Some(Nop(*arg)),
            Nop(arg) => Some(Jmp(*arg)),
            Acc(_) => None,
        } {
            let mutated_program =
                [&program[..i], &[mutated_instruction], &program[i + 1..]].concat();
            match find_infinite_loop(&mutated_program) {
                Either::Left(state) => {}
                Either::Right(state) => {
                    println!("  - Found a mutated program (i = {}) that terminates!", i);
                    println!("  - Last state: {:?}", state);
                    break;
                }
            };
        }
    }

    Ok(())
}
