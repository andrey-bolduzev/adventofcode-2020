#[macro_use]
extern crate scan_fmt;

use aoc2020::parse_newline_sep;

use itertools::Itertools as _;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut tiles = HashMap::new();
    for block in parse_newline_sep::<String>(&input)? {
        let mut lines = block.trim().lines();
        let (_, id) = scan_fmt!(lines.next().unwrap(), "{} {d}:", String, usize).unwrap();
        let lines = lines.map(|s| s.to_owned()).collect_vec();
        let tile = Tile::new(lines);
        tiles.insert(id, tile);
    }

    let mut corner = vec![];

    for (id, tile) in tiles.iter() {
        if tile.is_corner(id, &tiles) {
            corner.push(*id);
        }
    }

    let product: usize = corner.iter().product();

    println!("part 1 {}", product);

    assert_eq!(15670959891893, product);

    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, PartialEq, Eq)]
struct Tile {
    edges: HashSet<String>,
}

impl Tile {
    fn new(lines: Vec<String>) -> Self {
        let top: String = lines[0].to_string();
        let bottom: String = lines.last().unwrap().to_string();
        let left: String = lines.iter().map(|l| l.chars().next().unwrap()).collect();
        let right: String = lines.iter().map(|l| l.chars().last().unwrap()).collect();

        let top_rev: String = top.chars().rev().collect();
        let bottom_rev: String = bottom.chars().rev().collect();
        let left_rev: String = left.chars().rev().collect();
        let right_rev: String = right.chars().rev().collect();

        let mut edges = HashSet::new();
        edges.insert(top);
        edges.insert(top_rev);
        edges.insert(bottom);
        edges.insert(bottom_rev);
        edges.insert(left);
        edges.insert(left_rev);
        edges.insert(right);
        edges.insert(right_rev);

        Self { edges }
    }

    fn is_corner(&self, self_id: &usize, all_tiles: &HashMap<usize, Tile>) -> bool {
        let mut unmatched = self.edges.clone();

        for (id, tile) in all_tiles.iter() {
            if id == self_id {
                continue;
            }

            unmatched = unmatched.difference(&tile.edges).cloned().collect();

            if unmatched.is_empty() {
                return false;
            }
        }

        unmatched.len() == 4
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
