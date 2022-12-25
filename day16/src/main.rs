use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::Instant;

use anyhow::{Error, Result};
use itertools::Itertools;
use once_cell_regex::regex;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Rule {
    field: String,
    ranges: (RangeInclusive<u32>, RangeInclusive<u32>),
}

impl Rule {
    fn contains(&self, value: &u32) -> bool {
        let (r1, r2) = &self.ranges;
        r1.contains(value) || r2.contains(value)
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"^([\w\s]+): (\d+)-(\d+) or (\d+)-(\d+)$");
        if let Some(caps) = re.captures(s) {
            let field = caps.get(1).unwrap().as_str().to_string();
            let min1 = caps.get(2).unwrap().as_str().parse()?;
            let max1 = caps.get(3).unwrap().as_str().parse()?;
            let min2 = caps.get(4).unwrap().as_str().parse()?;
            let max2 = caps.get(5).unwrap().as_str().parse()?;
            Ok(Rule {
                field,
                ranges: (min1..=max1, min2..=max2),
            })
        } else {
            Err(anyhow::anyhow!("No match for `{}`", s))
        }
    }
}

#[derive(Debug)]
struct Ticket {
    numbers: Vec<u32>,
}

impl FromStr for Ticket {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s.split(',').map(|t| t.parse()).try_collect()?;
        Ok(Ticket { numbers })
    }
}

fn solve_part_one(rules: &[Rule], nearby_tickets: &[Ticket]) -> u32 {
    let ranges = rules
        .iter()
        .flat_map(|r| once(&r.ranges.0).chain(once(&r.ranges.1)))
        .collect_vec();
    println!("All ranges: {:?}", ranges);
    let rate: u32 = nearby_tickets
        .iter()
        .flat_map(|t| t.numbers.clone())
        .filter(|x| ranges.iter().all(|r| !r.contains(x)))
        .sum();
    println!("Ticket scanning error rate: {}", rate);
    rate
}

fn find_field_values<'a>(
    rules: &'a [Rule],
    tickets: &[&Ticket],
    values: &[u32],
) -> HashMap<&'a Rule, u32> {
    let mut field_value = HashMap::new();
    let mut used_positions = HashSet::new();
    // Note: one can omit the variable `used_positions` and replace the check
    //  `used_positions.contains(&i)` with `field_value.values().any(|&j| j == i)`.
    // Here, simpler solution with extra HashSet variable was chosen.

    while field_value.len() < rules.len() {
        let mut found_any = false;
        for rule in rules.iter() {
            // Skip rules with already inferred positions
            if field_value.contains_key(rule) {
                continue;
            }
            // Find possible positions for the field `rule.field`
            let possible = values
                .iter()
                .enumerate()
                .filter(|&(i, _)| {
                    tickets
                        .iter()
                        .all(|t| rule.contains(&t.numbers[i]) && !used_positions.contains(&i))
                })
                .collect_vec();
            // If the possible position is the only one, then save and use it
            if possible.len() == 1 {
                let (position, &value) = possible[0];
                field_value.insert(rule, value);
                used_positions.insert(position);
                found_any = true;
            }
        }
        // Note: in general, this problem can have either many or no solutions,
        //  but we are given the inputs for which the problem is guaranteed
        //  to have only one solution, which can be found iteratively by the presented loop.
        // If only something gone wrong, this check just aborts the infinite loop;
        //  in this situation, check your inputs!
        if !found_any {
            unreachable!("Stale");
        }
    }

    field_value
}

fn solve_part_two(rules: &[Rule], my_ticket: &Ticket, nearby_tickets: &[Ticket]) -> u64 {
    let valid_tickets = nearby_tickets
        .iter()
        .filter(|t| {
            t.numbers
                .iter()
                .all(|x| rules.iter().any(|r| r.contains(x)))
        })
        .chain(once(my_ticket))
        .collect_vec();

    let field_value = find_field_values(&rules, &valid_tickets, &my_ticket.numbers);

    let product: u64 = rules
        .iter()
        .filter(|r| r.field.starts_with("departure"))
        .map(|r| field_value[r] as u64)
        .product();
    println!("Product of values for `departure`-fields: {}", product);
    product
}

fn read_rules(lines: &mut impl Iterator<Item = String>) -> Vec<Rule> {
    lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect_vec()
}

fn read_my_ticket(lines: &mut impl Iterator<Item = String>) -> Ticket {
    let header = lines.next().unwrap();
    assert_eq!("your ticket:", header);

    let ticket = lines.next().unwrap().parse().unwrap();

    let empty_line = lines.next().unwrap();
    assert!(empty_line.is_empty());

    ticket
}

fn read_nearby_tickets(lines: &mut impl Iterator<Item = String>) -> Vec<Ticket> {
    let header = lines.next().unwrap();
    assert_eq!("nearby tickets:", header);

    lines.map(|line| line.parse().unwrap()).collect_vec()
}

fn main() -> Result<()> {
    let start_time = Instant::now();
    let path = "data/input.txt";
    // let path = "data/sample1.txt"; // 71 for part 1
    // let path = "data/sample2.txt"; // (no answer) for part 2
    let mut lines = BufReader::new(File::open(path)?)
        .lines()
        .map(|r| r.unwrap());
    let rules = read_rules(&mut lines);
    let my_ticket = read_my_ticket(&mut lines);
    let nearby_tickets = read_nearby_tickets(&mut lines);

    // println!("rules: {:?}", rules);
    // println!("my ticket: {:?}", my_ticket);
    // println!("nearby tickets: {:?}", nearby_tickets);

    println!("Solving part 1...");
    solve_part_one(&rules, &nearby_tickets);
    println!();
    println!("Solving part 2...");
    solve_part_two(&rules, &my_ticket, &nearby_tickets);

    println!("All done in {:.2} s", start_time.elapsed().as_secs_f32());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample1() {
        let path = "data/sample1.txt";
        let mut lines = BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|r| r.unwrap());
        let rules = read_rules(&mut lines);
        let _my_ticket = read_my_ticket(&mut lines);
        let nearby_tickets = read_nearby_tickets(&mut lines);
        let result = solve_part_one(&rules, &nearby_tickets);
        assert_eq!(result, 71);
    }

    #[test]
    fn test_part_one_input() {
        let path = "data/input.txt";
        let mut lines = BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|r| r.unwrap());
        let rules = read_rules(&mut lines);
        let _my_ticket = read_my_ticket(&mut lines);
        let nearby_tickets = read_nearby_tickets(&mut lines);
        let result = solve_part_one(&rules, &nearby_tickets);
        assert_eq!(result, 21980);
    }

    #[test]
    fn test_part_two_input() {
        let path = "data/input.txt";
        let mut lines = BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|r| r.unwrap());
        let rules = read_rules(&mut lines);
        let my_ticket = read_my_ticket(&mut lines);
        let nearby_tickets = read_nearby_tickets(&mut lines);
        let result = solve_part_two(&rules, &my_ticket, &nearby_tickets);
        assert_eq!(result, 1439429522627);
    }
}
