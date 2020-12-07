#[macro_use]
extern crate scan_fmt;

use aoc2020::parse;
use itertools::Itertools as _;
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let bags = parse::<Bag>(&input)?.collect_vec();

    let count = find_containers(&bags, "shiny gold".to_string())
        .iter()
        .unique()
        .count();

    println!("part 1 {}", count);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let bags = parse::<Bag>(&input)?.collect_vec();

    let count = count_descendants(
        &bags,
        bags.iter().find(|c| c.color == "shiny gold").unwrap(),
    );

    println!("part 2 {}", count);

    Ok(())
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Bag {
    color: String,
    can_directly_contain: Vec<ColorAndNumber>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct ColorAndNumber {
    color: String,
    number: usize,
}

fn find_containers(all_bags: &[Bag], target: String) -> Vec<Bag> {
    let mut containers: Vec<Bag> = all_bags
        .iter()
        .filter(|&bag| bag.can_directly_contain_color(&target))
        .cloned()
        .collect_vec();

    if containers.is_empty() {
        vec![]
    } else {
        let mut recurse =
            containers
                .iter()
                .map(|bag| bag.color.to_string())
                .fold(vec![], |mut acc, c| {
                    acc.append(&mut find_containers(all_bags, c));
                    acc
                });
        containers.append(&mut recurse);
        containers
    }
}

fn count_descendants(all_bags: &[Bag], start: &Bag) -> usize {
    let descendants = &start.can_directly_contain;
    if descendants.is_empty() {
        0
    } else {
        descendants.iter().fold(0, |acc, b| {
            acc + b.number
                + b.number
                    * count_descendants(
                        all_bags,
                        all_bags.iter().find(|c| c.color == b.color).unwrap(),
                    )
        })
    }
}

impl Bag {
    fn can_directly_contain_color(&self, target: &str) -> bool {
        self.can_directly_contain
            .iter()
            .map(|c| c.color.to_string())
            .collect_vec()
            .contains(&target.to_string())
    }
}

impl std::str::FromStr for Bag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let parts: Vec<String> = s.split(" bags contain ").map(|s| s.to_string()).collect();
        let color = &parts[0];

        let contents = &parts[1];
        let in_bags: Vec<ColorAndNumber>;

        if contents == "no other bags." {
            in_bags = vec![];
        } else {
            in_bags = contents
                .split(',')
                .map(|s| s.trim())
                .fold(vec![], |mut acc, c| {
                    let (number, color1, color2) =
                        scan_fmt!(c, "{} {} {} bags", usize, String, String).unwrap();

                    acc.push(ColorAndNumber {
                        color: color1 + " " + &color2,
                        number,
                    });
                    acc
                });
        }
        Ok(Bag {
            color: color.to_string(),
            can_directly_contain: in_bags,
        })
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
