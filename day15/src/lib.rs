use aoc2020::parse;
use aoc2020::CommaSep;

use itertools::Itertools as _;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let answer = solve(&parse_input(&input), 2020);

    println!("part 1 {}", answer);

    assert_eq!(1025, answer);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let answer = solve(&parse_input(&input), 30000000);

    println!("part 2 {}", answer);

    assert_eq!(129262, answer);

    Ok(())
}

fn parse_input(input: &Path) -> Vec<usize> {
    parse::<CommaSep<usize>>(input)
        .unwrap()
        .next()
        .unwrap()
        .into_iter()
        .collect_vec()
}

fn solve(input: &[usize], count: usize) -> usize {
    let mut last_seen: HashMap<usize, usize> = input
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, item)| (item, index))
        .collect();

    (input.len()..count).fold(*input.last().unwrap(), |last, index| {
        match last_seen.entry(last) {
            Entry::Occupied(mut occ) => index - occ.insert(index - 1) - 1,
            Entry::Vacant(vac) => {
                vac.insert(index - 1);
                0
            }
        }
    })
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
