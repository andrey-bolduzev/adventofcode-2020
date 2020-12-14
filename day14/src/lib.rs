use aoc2020::parse;

use itertools::Itertools as _;
use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut current_mask: String = "".to_string();
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for instruction in parse::<Instructions>(&input)? {
        match instruction {
            Instructions::Mask { value } => current_mask = value,
            Instructions::MemoryWrite { address, value } => {
                *memory.entry(address).or_default() = apply_mask(&current_mask, value);
            }
        }
    }

    let answer: usize = memory.values().sum();

    println!("part 1 {}", answer);

    assert_eq!(12408060320841, answer);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut current_mask: String = "".to_string();
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for instruction in parse::<Instructions>(&input)? {
        match instruction {
            Instructions::Mask { value } => current_mask = value,
            Instructions::MemoryWrite { address, value } => {
                let addresses_to_write = apply_mask_to_address(&current_mask, address);

                addresses_to_write
                    .iter()
                    .for_each(|&addr| *memory.entry(addr).or_default() = value);
            }
        }
    }

    let answer: usize = memory.values().sum();

    println!("part 2 {}", answer);

    assert_eq!(4466434626828, answer);

    Ok(())
}

fn apply_mask(mask: &str, value: usize) -> usize {
    let mut value_binary = format!("{:036b}", value);

    for (index, value) in mask.chars().enumerate().filter(|(_, n)| *n != 'X') {
        match value {
            '0' => value_binary.replace_range(index..index + 1, "0"),
            '1' => value_binary.replace_range(index..index + 1, "1"),
            _ => panic!("value {} shouldn't happen here", value),
        }
    }

    usize::from_str_radix(&value_binary, 2).unwrap()
}

fn apply_mask_to_address(mask: &str, address: usize) -> Vec<usize> {
    let value_binary = format!("{:036b}", address);
    let mut results: Vec<String> = vec![value_binary];

    for (index, value) in mask.chars().enumerate() {
        match value {
            '0' => {}
            '1' => {
                for result in results.iter_mut() {
                    result.replace_range(index..index + 1, "1");
                }
            }
            'X' => {
                let mut new_results = HashSet::new();
                for result in results.iter_mut() {
                    result.replace_range(index..index + 1, "0");
                    let mut copy = result.clone();
                    copy.replace_range(index..index + 1, "1");
                    new_results.insert(copy);
                }
                results.extend(new_results);
            }
            _ => panic!("value {} shouldn't happen here", value),
        }
    }

    results
        .iter()
        .map(|r| usize::from_str_radix(&r, 2).unwrap())
        .collect_vec()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, FromStr, Display)]
enum Instructions {
    #[display("mask = {value}")]
    Mask { value: String },
    #[display("mem[{address}] = {value}")]
    MemoryWrite { address: usize, value: usize },
}
