use aoc2020::{geometry::Point, parse};

use itertools::Itertools as _;
use std::path::Path;
use thiserror::Error;

const LINE_LENGTH: usize = 31;
const LAST_LINE_INDEX: i32 = 322;

pub fn part1(input: &Path) -> Result<(), Error> {
    let lines: Vec<String> = parse::<String>(&input)?.collect_vec();

    let tree_count = traverse_slope(&lines, 3, 1);

    println!("part 1 {}", tree_count);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let lines: Vec<String> = parse(&input)?.collect_vec();

    let result = traverse_slope(&lines, 1, 1)
        * traverse_slope(&lines, 3, 1)
        * traverse_slope(&lines, 5, 1)
        * traverse_slope(&lines, 7, 1)
        * traverse_slope(&lines, 1, 2);

    println!("part 2 {}", result);

    Ok(())
}

fn traverse_slope(lines: &[String], delta_x: i32, delta_y: i32) -> usize {
    let mut current = Point::new(0, 0);
    let mut trace: Vec<Point> = Vec::new();

    loop {
        if current.y >= LAST_LINE_INDEX {
            break;
        }
        current += Point::new(delta_x, delta_y);
        trace.push(current);
    }

    let mut tree_count = 0;

    for point in trace.iter() {
        let x: usize = point.x as usize;
        let y: usize = point.y as usize;

        if lines
            .get(y as usize)
            .unwrap()
            .chars()
            .nth(x as usize % LINE_LENGTH)
            .unwrap()
            == '#'
        {
            tree_count += 1;
        }
    }

    tree_count
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
