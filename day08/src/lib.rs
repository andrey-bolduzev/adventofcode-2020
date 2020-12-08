use aoc2020::parse;

use itertools::Itertools as _;
use parse_display::{Display, FromStr};
use std::path::Path;
use thiserror::Error;
use Finiteness::*;

pub fn part1(input: &Path) -> Result<(), Error> {
    let instructions = parse::<Instructions>(&input)?.collect_vec();

    if let Infinite { acc } = test_finite(&instructions) {
        println!("part 1 {}", acc);

        assert_eq!(1475, acc);

        return Ok(());
    }

    panic!("execution should've been infinite but was finite");
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let instructions = parse::<Instructions>(&input)?.collect_vec();

    let answer;
    let mut current_index = 0;

    loop {
        let mut attempt = instructions.clone();

        match attempt[current_index] {
            Instructions::Nop { value } => {
                attempt[current_index] = Instructions::Jmp { value };
                if let Finite { acc } = test_finite(&attempt) {
                    answer = acc;
                    break;
                }
                current_index += 1;
            }
            Instructions::Jmp { value } => {
                attempt[current_index] = Instructions::Nop { value };
                if let Finite { acc } = test_finite(&attempt) {
                    answer = acc;
                    break;
                }
                current_index += 1;
            }
            _ => current_index += 1,
        }
    }

    println!("part 2 {}", answer);

    assert_eq!(1270, answer);

    Ok(())
}

fn test_finite(instructions: &[Instructions]) -> Finiteness {
    let mut acc: i32 = 0;
    let mut visited: Vec<usize> = vec![0];

    loop {
        let index = *visited.last().unwrap();
        let current_ins = instructions.get(index);

        if current_ins.is_none() {
            return Finite { acc };
        }

        let current_ins = current_ins.unwrap();

        let next_index: usize;
        match current_ins {
            Instructions::Nop { .. } => next_index = index + 1,
            Instructions::Acc { value } => {
                next_index = index + 1;
                acc += value;
            }
            Instructions::Jmp { value } => {
                next_index = (index as i32 + value) as usize;
            }
        }

        if visited.contains(&next_index) {
            return Infinite { acc };
        }

        visited.push(next_index);
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
enum Instructions {
    #[display("nop {value}")]
    Nop { value: i32 },

    #[display("acc {value}")]
    Acc { value: i32 },

    #[display("jmp {value}")]
    Jmp { value: i32 },
}

enum Finiteness {
    Finite { acc: i32 },
    Infinite { acc: i32 },
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
