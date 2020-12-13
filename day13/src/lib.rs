use aoc2020::parse;
use aoc2020::CommaSep;

use itertools::Itertools as _;
use ring_algorithm::chinese_remainder_theorem;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut input = parse::<String>(&input)?;
    let epoch: usize = input.next().unwrap().parse().unwrap();
    let schedule = input.next().unwrap();
    let schedule: Vec<usize> = CommaSep::<String>::from_str(&schedule)
        .unwrap()
        .into_iter()
        .filter(|b| b != "x")
        .map(|b| b.parse::<usize>().unwrap())
        .collect_vec();

    let mut min_wait = usize::MAX;
    let mut min_bus = 0;

    for bus in schedule.iter() {
        let wait = ((epoch % bus) as i32 - (*bus as i32)).abs() as usize;
        if min_wait > wait {
            min_wait = wait;
            min_bus = *bus;
        }
    }

    println!("part 1 {}", min_bus * min_wait);

    assert_eq!(2545, min_bus * min_wait);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut input = parse::<String>(&input)?;
    input.next().unwrap();
    let schedule = input.next().unwrap();
    let schedule: Vec<(i64, i64)> = CommaSep::<String>::from_str(&schedule)
        .unwrap()
        .into_iter()
        .enumerate()
        .filter(|(_, b)| b != "x")
        .map(|(o, b)| (o as i64, b.parse::<i64>().unwrap()))
        .collect_vec();

    let u = schedule.iter().map(|(o, b)| b - o).collect_vec();
    let m: Vec<i64> = schedule.iter().map(|(_, b)| b).cloned().collect_vec();
    let answer = chinese_remainder_theorem::<i64>(&u, &m).unwrap();

    println!("part 2 {}", answer);

    assert_eq!(266204454441577, answer);

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
