//! This is a parser for math expressions,
//! where addition (+,-) and multiplication (*,/)
//! have different, but **reversed** precedence.
//! For example, 1*2+3 = 1*(2+3) = 5

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::map_res;
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair};
use nom::Finish;

type Parsed<'a, T> = nom::IResult<&'a str, T>;

fn parens(i: &str) -> Parsed<i64> {
    delimited(tag("("), expr, tag(")"))(i)
}

fn number(i: &str) -> Parsed<i64> {
    map_res(digit1, |s: &str| s.parse())(i)
}

fn primary(i: &str) -> Parsed<i64> {
    delimited(multispace0, alt((number, parens)), multispace0)(i)
}

fn term(i: &str) -> Parsed<i64> {
    primary(i)
}

fn factor(i: &str) -> Parsed<i64> {
    let (i, init) = term(i)?;
    fold_many0(
        pair(alt((char('+'), char('-'))), term),
        init,
        |acc, (op, expr)| match op {
            '+' => acc + expr,
            '-' => acc - expr,
            _ => unreachable!(),
        },
    )(i)
}

fn expr(i: &str) -> Parsed<i64> {
    let (i, init) = factor(i)?;
    fold_many0(
        pair(alt((char('*'), char('/'))), factor),
        init,
        |acc, (op, expr)| match op {
            '*' => acc * expr,
            '/' => acc / expr,
            _ => unreachable!(),
        },
    )(i)
}

pub(crate) fn parse(line: &str) -> i64 {
    let (unparsed, value) = expr(line).finish().unwrap();
    assert!(unparsed.is_empty());
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "1 + 2 * 3";
        assert_eq!(9, parse(input));

        let input = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(231, parse(input));

        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(51, parse(input));

        let input = "2 * 3 + (4 * 5)";
        assert_eq!(46, parse(input));

        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(1445, parse(input));

        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(669060, parse(input));

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(23340, parse(input));
    }
}
