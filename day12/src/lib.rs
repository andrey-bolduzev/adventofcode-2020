use aoc2020::geometry::*;
use aoc2020::parse;

use itertools::Itertools as _;
use parse_display::{Display, FromStr};
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut current_position = Position {
        point: Point::default(),
        direction: Direction::Right,
    };

    for instruction in parse::<Instructions>(&input)? {
        match instruction {
            Instructions::North { value } => {
                current_position.point += Point::from(Direction::Up.deltas()) * value as i32
            }
            Instructions::East { value } => {
                current_position.point += Point::from(Direction::Right.deltas()) * value as i32
            }
            Instructions::South { value } => {
                current_position.point += Point::from(Direction::Down.deltas()) * value as i32
            }
            Instructions::West { value } => {
                current_position.point += Point::from(Direction::Left.deltas()) * value as i32
            }
            Instructions::Forward { value } => {
                let deltas = current_position.direction.deltas();
                current_position.point += Point::from(deltas) * value as i32;
            }
            Instructions::Right { value } => match value {
                90 => current_position.direction = current_position.direction.turn_right(),
                180 => {
                    current_position.direction =
                        current_position.direction.turn_right().turn_right();
                }
                270 => current_position.direction = current_position.direction.turn_left(),
                _ => panic!("no such input {}", value),
            },
            Instructions::Left { value } => match value {
                90 => current_position.direction = current_position.direction.turn_left(),
                180 => {
                    current_position.direction = current_position.direction.turn_left().turn_left();
                }
                270 => current_position.direction = current_position.direction.turn_right(),
                _ => panic!("no such input {}", value),
            },
        }
    }

    let distance = current_position.point.manhattan();

    println!("part 1 {}", distance);

    assert_eq!(2270, distance);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut current_position = Position2 {
        ship: Point::default(),
        wp: Point::new(10, 1),
    };

    for instruction in parse::<Instructions>(&input)? {
        match instruction {
            Instructions::North { value } => {
                current_position.wp += Point::from(Direction::Up.deltas()) * value as i32
            }
            Instructions::East { value } => {
                current_position.wp += Point::from(Direction::Right.deltas()) * value as i32
            }
            Instructions::South { value } => {
                current_position.wp += Point::from(Direction::Down.deltas()) * value as i32
            }
            Instructions::West { value } => {
                current_position.wp += Point::from(Direction::Left.deltas()) * value as i32
            }
            Instructions::Forward { value } => {
                current_position.ship += current_position.wp * value as i32;
            }
            Instructions::Right { value } => match value {
                90 => {
                    current_position.wp = Point::new(current_position.wp.y, -current_position.wp.x)
                }
                180 => {
                    current_position.wp = Point::new(-current_position.wp.x, -current_position.wp.y)
                }
                270 => {
                    current_position.wp = Point::new(-current_position.wp.y, current_position.wp.x)
                }
                _ => panic!("no such input {}", value),
            },
            Instructions::Left { value } => match value {
                90 => {
                    current_position.wp = Point::new(-current_position.wp.y, current_position.wp.x)
                }
                180 => {
                    current_position.wp = Point::new(-current_position.wp.x, -current_position.wp.y)
                }
                270 => {
                    current_position.wp = Point::new(current_position.wp.y, -current_position.wp.x)
                }
                _ => panic!("no such input {}", value),
            },
        }
    }

    let distance = current_position.ship.manhattan();

    println!("part 2 {}", distance);

    assert_eq!(138669, distance);

    Ok(())
}

struct Position {
    point: Point,
    direction: Direction,
}

struct Position2 {
    ship: Point,
    wp: Point,
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Instructions {
    #[display("N{value}")]
    North { value: usize },

    #[display("E{value}")]
    East { value: usize },

    #[display("S{value}")]
    South { value: usize },

    #[display("W{value}")]
    West { value: usize },

    #[display("F{value}")]
    Forward { value: usize },

    #[display("R{value}")]
    Right { value: usize },

    #[display("L{value}")]
    Left { value: usize },
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
