use aoc2020::parse;
use combinations::Combinations;
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let numbers: Vec<usize> = parse(input)?.collect();
    let pairs: Vec<_> = Combinations::new(numbers, 2).collect();
    let pair = pairs.iter().find(|pair| pair[0] + pair[1] == 2020).unwrap();
    println!("part1: {}", pair[0] * pair[1]);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let numbers: Vec<usize> = parse(input).unwrap().collect();
    let tuples: Vec<_> = Combinations::new(numbers, 3).collect();
    let tuple = tuples
        .iter()
        .find(|tuple| tuple[0] + tuple[1] + tuple[2] == 2020)
        .unwrap();
    println!("part2: {}", tuple[0] * tuple[1] * tuple[2]);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("couldn't parse numbers")]
    Io(#[from] std::io::Error),
}
