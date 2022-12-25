use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use tap::Pipe;

use crate::rule::RuleLine;

mod parser;
mod rule;

fn solve_part_one(rules: &[RuleLine], messages: &[String]) -> usize {
    let rulemap: HashMap<_, _> = rules.iter().map(|r| (r.index, &r.rule)).collect();
    let re_str = rulemap[&0]
        .to_regex_string(&rulemap)
        .pipe(|s| format!(r"^{}$", s));
    // println!("Regex: {:?}", re_str);
    let re = Regex::new(&re_str).unwrap();

    let count = messages.iter().filter(|m| re.is_match(m)).count();
    println!("Total matching messages: {}", count);
    count
}

fn solve_part_two(rules: &[RuleLine], messages: &[String]) -> usize {
    let rulemap: HashMap<_, _> = rules.iter().map(|r| (r.index, &r.rule)).collect();

    let re_str = {
        let mut cache = HashMap::new();
        let rule31 = &rulemap[&31];
        let rule31_re_str = rule31.to_regex_string_with_cache(&rulemap, &mut cache);
        cache.insert(31, rule31_re_str.clone());
        let rule42 = &rulemap[&42];
        let rule42_re_str = rule42.to_regex_string_with_cache(&rulemap, &mut cache);
        cache.insert(42, rule42_re_str.clone());

        // 8 ::= 42 | 42 8
        // This is just a regular expression r"(42)+"
        let new_rule8_re_str = format!("({})+", rule42_re_str);
        cache.insert(8, new_rule8_re_str);

        // 11 ::= 42 31 | 42 11 31
        // This cannot be represented by a regular expression,
        //  because we need something like r"42{i} 11* 31{j}", where `i=j`,
        //  but regular expressions do not have "memory", so we can't express `i=j`.
        // However, here I use dirty hack by encoding the necessary "non-regular expression"
        //  as "regular" alternation of the following form:
        //    r"42 31 | 42 42 31 31 | ... | 42{20} 31{20}",
        //  where both 42 and 31 are repeated the same number of times.
        // Note that the upper bound "20" is purely heuristic and seems to "just work".
        // If it doesn't, try to increase it, but keep in mind that the resulting regex
        //  could not be compilable due to high memory usage.
        // For example, using "50" as an upper bound, panics with CompiledTooBig error.
        // In order to reduce the memory usage, non-capturing groups (namely, r"(?:)")
        //  were used, but anyway, compiled regexes end up being very large...
        // Also note that here we pre-fill the cache and heavily rely on the fact
        //  (given in the problem statement) that rules are acyclic.
        let new_rule11_re_str = (1..=20)
            .map(|i| {
                format!(
                    "(?:{}){{{}}}(?:{}){{{}}}",
                    rule42_re_str, i, rule31_re_str, i
                )
            })
            .join("|")
            .pipe(|s| format!("(?:{})", s));
        // println!("new_rule11_re_str = {}", new_rule11_re_str);
        cache.insert(11, new_rule11_re_str);

        rulemap[&0]
            .to_regex_string_with_cache(&rulemap, &mut cache)
            .pipe(|s| format!(r"^{}$", s))
    };
    // println!("Regex: {:?}", re_str);
    let re = Regex::new(&re_str).unwrap();

    let count = messages.iter().filter(|m| re.is_match(m)).count();
    println!("Total matching messages: {}", count);
    count
}

fn read_data(path: &str) -> Result<(Vec<RuleLine>, Vec<String>)> {
    let mut lines = BufReader::new(File::open(path)?)
        .lines()
        .map(|r| r.expect("Could not read line"));
    let mut rules = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        rules.push(line.parse::<RuleLine>()?);
    }
    let messages = lines.collect_vec();

    Ok((rules, messages))
}

fn main() -> Result<()> {
    let start_time = Instant::now();
    let path = "data/input.txt";
    // let path = "data/sample.txt"; // 2
    let (rules, messages) = read_data(path)?;

    // println!("rules ({}):", rules.len());
    // let rulemap = rules.iter().map(|r| (r.index, &r.rule)).collect();
    // for rule in rules.iter().sorted_by_key(|r| r.index) {
    //     println!(" - {:?} == {}", rule, rule.rule.to_regex_string(&rulemap));
    // }
    // println!("messages ({}):", messages.len());
    // for msg in messages.iter() {
    //     println!(" - {:?}", msg);
    // }

    println!("Solving part one...");
    solve_part_one(&rules, &messages);
    println!();
    println!("Solving part two...");
    solve_part_two(&rules, &messages);

    println!("All done in {:.2} s", start_time.elapsed().as_secs_f32());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let path = "data/sample.txt";
        let (rules, messages) = read_data(path).unwrap();
        let result = solve_part_one(&rules, &messages);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_sample2_part_two() {
        let path = "data/sample2.txt";
        let (rules, messages) = read_data(path).unwrap();
        let result = solve_part_two(&rules, &messages);
        assert_eq!(result, 12);
    }
}
