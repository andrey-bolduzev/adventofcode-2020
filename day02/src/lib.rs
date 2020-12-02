use aoc2020::parse;

use std::path::Path;
use thiserror::Error;
use itertools::Itertools as _;

pub fn part1(input: &Path) -> Result<(), Error> {
    unimplemented!()
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
