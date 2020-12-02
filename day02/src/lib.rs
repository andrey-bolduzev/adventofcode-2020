use aoc2020::parse;

use parse_display::{Display, FromStr};
use std::path::Path;
use thiserror::Error;

#[derive(Display, FromStr, Debug)]
#[display("{first_position}-{second_position} {symbol}: {password}")]
struct PasswordPolicy {
    first_position: usize,
    second_position: usize,
    symbol: char,
    password: String,
}

impl PasswordPolicy {
    fn is_valid_part1(&self) -> bool {
        let occurences = self.password.chars().filter(|&c| c == self.symbol).count();
        (self.first_position..=self.second_position).contains(&occurences)
    }

    fn is_valid_part2(&self) -> bool {
        let compare_nth =
            |position: usize| self.password.chars().nth(position - 1).unwrap() == self.symbol;
        compare_nth(self.first_position) ^ compare_nth(self.second_position)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let answer = parse::<PasswordPolicy>(&input)?
        .into_iter()
        .filter(PasswordPolicy::is_valid_part1)
        .count();

    println!("part 1 {}", answer);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let answer = parse::<PasswordPolicy>(&input)?
        .into_iter()
        .filter(PasswordPolicy::is_valid_part2)
        .count();

    println!("part 2 {}", answer);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}
