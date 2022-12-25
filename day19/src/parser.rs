use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, digit1, space1};
use nom::combinator::{map, map_res};
use nom::sequence::{delimited, tuple};
use nom::Finish;

use crate::rule::RuleLine;

use super::rule::Rule;
use nom::multi::separated_list1;

type Parsed<'a, T> = nom::IResult<&'a str, T>;

fn index(i: &str) -> Parsed<usize> {
    map_res(digit1, |x: &str| x.parse::<usize>())(i)
}

fn string(i: &str) -> Parsed<String> {
    let (i, s) = delimited(char('"'), is_not("\""), char('"'))(i)?;
    Ok((i, s.to_string()))
}

fn rule_alt(i: &str) -> Parsed<Vec<Vec<usize>>> {
    separated_list1(tag(" | "), separated_list1(space1, index))(i)
}

fn rule(i: &str) -> Parsed<Rule> {
    alt((
        map(string, |s| Rule::Match(s)),
        map(rule_alt, |x| Rule::Alt(x)),
    ))(i)
}

fn ruleline(i: &str) -> Parsed<RuleLine> {
    map(tuple((index, tag(": "), rule)), |(index, _, rule)| {
        RuleLine { index, rule }
    })(i)
}

pub(crate) fn parse_ruleline(line: &str) -> RuleLine {
    let (unparsed, value) = ruleline(line).finish().unwrap();
    assert!(unparsed.is_empty());
    value
}
