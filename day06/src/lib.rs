use aoc2020::*;

use itertools::Itertools as _;
use std::collections::HashSet;
use std::path::Path;
use std::str::*;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let count: usize = parse_newline_sep::<Group>(&input)?
        .map(|g| g.count_any_yes())
        .sum();

    println!("part 1 {}", count);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let count: usize = parse_newline_sep::<Group>(&input)?
        .map(|g| g.count_all_yes())
        .sum();

    println!("part 2 {}", count);
    Ok(())
}

struct Group {
    answers: Vec<String>,
}

impl FromStr for Group {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.split_whitespace().map(|s| s.to_owned()).collect(),
        ))
    }
}

impl Group {
    fn new(answers: Vec<String>) -> Self {
        Self { answers }
    }

    fn count_any_yes(&self) -> usize {
        let all_answers: Vec<_> = self.answers.iter().flat_map(|ans| ans.chars()).collect();
        all_answers.into_iter().unique().count()
    }

    fn count_all_yes(&self) -> usize {
        let mut all = self
            .answers
            .iter()
            .map(|string| string.chars().collect::<HashSet<char>>());

        let intersection = all
            .next()
            .map(|first| all.fold(first, |set1, set2| &set1 & &set2))
            .unwrap();

        intersection.len()
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
