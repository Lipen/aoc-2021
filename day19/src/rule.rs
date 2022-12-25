use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Error;
use itertools::Itertools;

use crate::parser::parse_ruleline;

#[derive(Debug, Clone)]
pub(crate) struct RuleLine {
    pub index: usize,
    pub rule: Rule,
}

impl FromStr for RuleLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_ruleline(s))
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Rule {
    Match(String),
    Alt(Vec<Vec<usize>>),
}

impl Rule {
    pub(crate) fn to_regex_string(&self, rulemap: &HashMap<usize, &Rule>) -> String {
        let mut cache = HashMap::new();
        self.to_regex_string_with_cache(rulemap, &mut cache)
    }

    pub(crate) fn to_regex_string_with_cache(
        &self,
        rulemap: &HashMap<usize, &Rule>,
        cache: &mut HashMap<usize, String>,
    ) -> String {
        // println!("Calculating regex for {:?}...", self);
        use Rule::*;
        match self {
            Match(s) => s.clone(),
            Alt(alts) => {
                let s = alts
                    .iter()
                    .map(|ixs| {
                        let s: String = ixs
                            .iter()
                            .map(|&i| {
                                if cache.contains_key(&i) {
                                    cache[&i].clone()
                                } else {
                                    let r = rulemap[&i].to_regex_string_with_cache(rulemap, cache);
                                    cache.insert(i, r.clone());
                                    r
                                }
                            })
                            .collect();
                        s
                    })
                    .join("|");
                if s.contains('|') {
                    format!("(?:{})", s)
                } else {
                    s
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_rule_to_regex() {
        let rule = Rule::Match("a".to_owned());

        let mut rulemap = HashMap::new();
        rulemap.insert(0, &rule);

        assert_eq!("a", rule.to_regex_string(&rulemap));
    }

    #[test]
    fn test_alt_rule_to_regex() {
        let rule = Rule::Alt(vec![vec![1], vec![2, 1]]);
        let rule1 = Rule::Match("a".to_owned());
        let rule2 = Rule::Match("b".to_owned());

        let mut rulemap = HashMap::new();
        rulemap.insert(0, &rule);
        rulemap.insert(1, &rule1);
        rulemap.insert(2, &rule2);

        assert_eq!("(a|ba)", rule.to_regex_string(&rulemap).replace("?:", ""));
    }

    #[test]
    fn test_nested_alt_rule_to_regex() {
        let rule = Rule::Alt(vec![vec![4, 1, 5]]);
        let rule1 = Rule::Alt(vec![vec![2, 3], vec![3, 2]]);
        let rule2 = Rule::Alt(vec![vec![4, 4], vec![5, 5]]);
        let rule3 = Rule::Alt(vec![vec![4, 5], vec![5, 4]]);
        let rule4 = Rule::Match("a".to_owned());
        let rule5 = Rule::Match("b".to_owned());

        let mut rulemap = HashMap::new();
        rulemap.insert(0, &rule);
        rulemap.insert(1, &rule1);
        rulemap.insert(2, &rule2);
        rulemap.insert(3, &rule3);
        rulemap.insert(4, &rule4);
        rulemap.insert(5, &rule5);

        assert_eq!(
            "(a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b)",
            rule.to_regex_string(&rulemap).replace("?:", "")
        );
    }
}
