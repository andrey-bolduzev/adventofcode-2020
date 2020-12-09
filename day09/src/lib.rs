use aoc2020::parse;

use combinations::Combinations;
use itertools::Itertools as _;
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let all_numbers: Vec<usize> = parse::<usize>(&input)?.collect_vec();

    let mut current_index = 25;

    loop {
        let preceeding = all_numbers
            .iter()
            .skip(current_index - 25)
            .take(25)
            .cloned()
            .collect_vec();

        let current_number = all_numbers[current_index];

        let combinations = Combinations::new(preceeding, 2).collect_vec();

        if combinations
            .iter()
            .find(|pair| current_number == pair[0] + pair[1])
            .is_none()
        {
            println!("part 1 {}", current_number);

            assert_eq!(1212510616, current_number);
            return Ok(());
        }

        current_index += 1;
    }
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let part_one_number = 1212510616;
    let all_numbers: Vec<usize> = parse::<usize>(&input)?.collect_vec();

    for i in 0..all_numbers.len() {
        let mut test_range = all_numbers.iter().skip(i).cloned().collect_vec();

        loop {
            if test_range.len() >= 2 && test_range.iter().sum::<usize>() == part_one_number {
                let min = test_range.iter().min().unwrap();
                let max = test_range.iter().max().unwrap();
                println!("part 2 {}", min + max);

                assert_eq!(171265123, min + max);
                return Ok(());
            }

            if test_range.pop().is_none() {
                break;
            }
        }
    }

    panic!("not found")
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
