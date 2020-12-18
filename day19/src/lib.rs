use aoc2020::parse_newline_sep;

use itertools::Itertools as _;
use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut blocks = parse_newline_sep::<String>(&input)?;
    let raw_rules = blocks.next().unwrap();
    let messages = blocks.next().unwrap();

    let count = solve_for(raw_rules, messages);

    println!("part 1 {}", count);

    assert_eq!(291, count);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut blocks = parse_newline_sep::<String>(&input)?;
    let raw_rules = blocks.next().unwrap();
    let raw_rules = raw_rules
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31");
    let messages = blocks.next().unwrap();

    let count = solve_for(raw_rules, messages);

    println!("part 2 {}", count);

    assert_eq!(409, count);

    Ok(())
}

fn solve_for(raw_rules: String, messages: String) -> usize {
    let raw_rules = raw_rules
        .trim()
        .lines()
        .into_iter()
        .map(|s| RawRule::from_str(s).unwrap())
        .collect_vec();

    let mut rules = HashMap::new();

    for raw_rule in raw_rules {
        let (index, rule) = parse(&raw_rule);
        rules.insert(index, rule);
    }

    let mut count = 0;
    for msg in messages.trim().lines() {
        let msg = msg.chars().collect_vec();
        for m in rules.get(&0).unwrap().matches(&rules, &msg).into_iter() {
            if m.is_empty() {
                count += 1;
                break;
            }
        }
    }

    count
}

fn parse(raw_rule: &RawRule) -> (usize, Rule) {
    match raw_rule {
        RawRule::Value { index, character } => (*index, Rule::Value(*character)),
        RawRule::OneRef { index, ref1 } => (*index, Rule::Ref(*ref1)),
        RawRule::TwoAndRefs { index, ref1, ref2 } => (
            *index,
            Rule::And(Box::new(Rule::Ref(*ref1)), Box::new(Rule::Ref(*ref2))),
        ),
        RawRule::TwoOrRefs { index, ref1, ref2 } => (
            *index,
            Rule::Or(Box::new(Rule::Ref(*ref1)), Box::new(Rule::Ref(*ref2))),
        ),
        RawRule::OneOrTwoRefs {
            index,
            ref1,
            ref2,
            ref3,
        } => (
            *index,
            Rule::Or(
                Box::new(Rule::Ref(*ref1)),
                Box::new(Rule::And(
                    Box::new(Rule::Ref(*ref2)),
                    Box::new(Rule::Ref(*ref3)),
                )),
            ),
        ),
        RawRule::TwoRefsOrTwoRefs {
            index,
            ref1,
            ref2,
            ref3,
            ref4,
        } => (
            *index,
            Rule::Or(
                Box::new(Rule::And(
                    Box::new(Rule::Ref(*ref1)),
                    Box::new(Rule::Ref(*ref2)),
                )),
                Box::new(Rule::And(
                    Box::new(Rule::Ref(*ref3)),
                    Box::new(Rule::Ref(*ref4)),
                )),
            ),
        ),
        RawRule::TwoRefsOrThreeRefs {
            index,
            ref1,
            ref2,
            ref3,
            ref4,
            ref5,
        } => (
            *index,
            Rule::Or(
                Box::new(Rule::And(
                    Box::new(Rule::Ref(*ref1)),
                    Box::new(Rule::Ref(*ref2)),
                )),
                Box::new(Rule::And3(
                    Box::new(Rule::Ref(*ref3)),
                    Box::new(Rule::Ref(*ref4)),
                    Box::new(Rule::Ref(*ref5)),
                )),
            ),
        ),
    }
}

#[derive(Debug, FromStr, Display)]
enum RawRule {
    #[display("{index}: \"{character}\"")]
    Value { index: usize, character: char },

    #[display("{index}: {ref1}")]
    OneRef { index: usize, ref1: usize },

    #[display("{index}: {ref1} {ref2}")]
    TwoAndRefs {
        index: usize,
        ref1: usize,
        ref2: usize,
    },

    #[display("{index}: {ref1} | {ref2}")]
    TwoOrRefs {
        index: usize,
        ref1: usize,
        ref2: usize,
    },

    #[display("{index}: {ref1} | {ref2} {ref3}")]
    OneOrTwoRefs {
        index: usize,
        ref1: usize,
        ref2: usize,
        ref3: usize,
    },

    #[display("{index}: {ref1} {ref2} | {ref3} {ref4}")]
    TwoRefsOrTwoRefs {
        index: usize,
        ref1: usize,
        ref2: usize,
        ref3: usize,
        ref4: usize,
    },

    #[display("{index}: {ref1} {ref2} | {ref3} {ref4} {ref5}")]
    TwoRefsOrThreeRefs {
        index: usize,
        ref1: usize,
        ref2: usize,
        ref3: usize,
        ref4: usize,
        ref5: usize,
    },
}

enum Rule {
    Ref(usize),
    Or(Box<Rule>, Box<Rule>),
    And(Box<Rule>, Box<Rule>),
    And3(Box<Rule>, Box<Rule>, Box<Rule>),
    Value(char),
}

impl Rule {
    fn matches<'a>(
        &self,
        rules: &'a HashMap<usize, Rule>,
        unparsed: &'a [char],
    ) -> Vec<&'a [char]> {
        if unparsed.is_empty() {
            return vec![];
        }

        match self {
            Rule::Ref(i) => rules.get(i).unwrap().matches(rules, unparsed),
            Rule::Or(a, b) => {
                let mut r = vec![];
                for a in a.matches(rules, unparsed) {
                    r.push(a);
                }
                for b in b.matches(rules, unparsed) {
                    r.push(b);
                }
                r
            }
            Rule::Value(c) => {
                if unparsed[0] == *c {
                    vec![&unparsed[1..]]
                } else {
                    vec![]
                }
            }
            Rule::And(a, b) => {
                let mut r = vec![];
                for m in a.matches(rules, unparsed) {
                    for n in b.matches(rules, m) {
                        r.push(n);
                    }
                }
                r
            }
            Rule::And3(a, b, c) => {
                let mut r = vec![];
                for m in a.matches(rules, unparsed) {
                    for n in b.matches(rules, m) {
                        for o in c.matches(rules, n) {
                            r.push(o);
                        }
                    }
                }
                r
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
