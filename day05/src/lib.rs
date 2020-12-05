#![feature(hash_drain_filter)]

use aoc2020::parse;

use itertools::Itertools as _;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

const NUM_ROWS: usize = 128;
const NUM_COLUMNS: usize = 8;

pub fn part1(input: &Path) -> Result<(), Error> {
    let max_id = parse_seats(input).iter().map(Seat::id).max().unwrap();

    println!("part 1 max id {}", max_id);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let seats = parse_seats(input);
    let mut layout: HashMap<usize, Vec<usize>> = HashMap::new();
    for seat in seats.iter() {
        let entry = layout.entry(seat.row).or_default();
        entry.push(seat.column);
    }

    let row_with_one_empty_seat: Vec<_> = layout
        .drain_filter(|_, v| v.len() == NUM_COLUMNS - 1)
        .collect();
    let (row, seats) = row_with_one_empty_seat.get(0).unwrap();
    let column = find_missing_column(seats);

    println!("part 2 my id {}", Seat { row: *row, column }.id());

    Ok(())
}

fn parse_seats(input: &Path) -> Vec<Seat> {
    let raw_seats = parse::<String>(&input)
        .unwrap()
        .map(RawSeat::new)
        .collect_vec();
    raw_seats.into_iter().map(Seat::from).collect_vec()
}

fn find_missing_column(row: &[usize]) -> usize {
    let total = (NUM_COLUMNS - 1) * 4;
    let sum: usize = row.iter().sum();
    total - sum
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
struct RawSeat {
    row: String,
    column: String,
}

impl RawSeat {
    pub fn new(line: String) -> Self {
        let row: String = line.chars().into_iter().take(7).collect();
        let column: String = line.chars().into_iter().skip(7).collect();
        Self { row, column }
    }
}

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
}

impl From<RawSeat> for Seat {
    fn from(raw: RawSeat) -> Self {
        let row = reduce_position(raw.row, 0, NUM_ROWS, 'F');
        let column = reduce_position(raw.column, 0, NUM_COLUMNS, 'L');
        Self { row, column }
    }
}

use num::integer::div_ceil;
use num::integer::div_floor;

fn reduce_position(mut instructions: String, low: usize, high: usize, low_char: char) -> usize {
    if instructions.is_empty() || low == high {
        low
    } else if instructions.remove(0) == low_char {
        reduce_position(instructions, low, div_floor(high + low, 2), low_char)
    } else {
        reduce_position(instructions, div_ceil(high + low, 2), high, low_char)
    }
}

impl Seat {
    fn id(&self) -> usize {
        self.row * NUM_COLUMNS + self.column
    }
}
