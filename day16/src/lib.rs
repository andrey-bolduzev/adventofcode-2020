#![feature(hash_drain_filter)]
use aoc2020::{parse, parse_newline_sep, CommaSep};

use itertools::Itertools as _;
use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut input_parts = parse_newline_sep::<String>(&input)?;
    let constraints = input_parts.next().unwrap();
    let _own_ticket = input_parts.next().unwrap();
    let other_tickets = input_parts.next().unwrap();

    let constraints: Vec<TicketConstraint> = constraints
        .trim()
        .lines()
        .map(|f| TicketConstraint::from_str(f).unwrap())
        .collect_vec();

    let other_tickets: Vec<RawTicket> = other_tickets
        .trim()
        .lines()
        .filter(|s| !s.starts_with("nearby"))
        .map(RawTicket::new)
        .collect_vec();

    let error: usize = other_tickets
        .into_iter()
        .flat_map(|t| t.fields)
        .filter(|&field| !constraints.iter().any(|c| c.is_valid(field)))
        .sum();

    println!("part 1 {}", error);

    assert_eq!(22057, error);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut input_parts = parse_newline_sep::<String>(&input)?;
    let constraints = input_parts.next().unwrap();
    let own_ticket = input_parts.next().unwrap();
    let other_tickets = input_parts.next().unwrap();

    let own_ticket = own_ticket
        .trim()
        .lines()
        .filter(|l| !l.starts_with("your"))
        .map(RawTicket::new)
        .next()
        .unwrap();

    let constraints: Vec<TicketConstraint> = constraints
        .trim()
        .lines()
        .map(|f| TicketConstraint::from_str(f).unwrap())
        .collect_vec();

    let other_tickets: Vec<RawTicket> = other_tickets
        .trim()
        .lines()
        .filter(|s| !s.starts_with("nearby"))
        .map(RawTicket::new)
        .collect_vec();

    let bad_fields = other_tickets
        .iter()
        .flat_map(|t| t.fields.clone())
        .filter(|&field| !constraints.iter().any(|c| c.is_valid(field)))
        .collect_vec();

    let other_tickets = other_tickets
        .into_iter()
        .filter(|raw| bad_fields.iter().all(|bad| !raw.fields.contains(bad)))
        .collect_vec();

    let mut candidates: HashMap<String, HashSet<usize>> = HashMap::new();

    for constraint in constraints.iter() {
        let mut candidate_indices = HashSet::new();
        let mut def_not = HashSet::new();
        for ticket in other_tickets.iter() {
            for (index, field) in ticket.fields.iter().enumerate() {
                if constraint.is_valid(*field) {
                    candidate_indices.insert(index);
                } else {
                    def_not.insert(index);
                }
            }
        }

        let candidate_indices: HashSet<_> = candidate_indices
            .into_iter()
            .filter(|i| !def_not.contains(&i))
            .collect();

        candidates.insert(constraint.name.clone(), candidate_indices);
    }

    let mut indices = vec![];
    while candidates.len() > 0 {
        let definite: Vec<_> = candidates
            .drain_filter(|_, v| v.len() == 1)
            .map(|(k, v)| (k, v.iter().next().unwrap().clone()))
            .collect();

        indices.extend(definite);

        for (_, ii) in candidates.iter_mut() {
            for (_, finished) in indices.iter() {
                if ii.contains(finished) {
                    ii.remove(finished);
                }
            }
        }
    }

    let answer = indices
        .into_iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, index)| index)
        .fold(1, |acc, next| acc * own_ticket.fields[next]);

    println!("part 2 {}", answer);

    assert_eq!(1093427331937, answer);

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
struct RawTicket {
    fields: Vec<usize>,
}

#[derive(Debug, FromStr, Display)]
#[display("{name}: {start1}-{end1} or {start2}-{end2}")]
struct TicketConstraint {
    name: String,
    start1: usize,
    end1: usize,
    start2: usize,
    end2: usize,
}

impl RawTicket {
    fn new(fields: &str) -> Self {
        let fields: Vec<usize> = CommaSep::<usize>::from_str(fields)
            .unwrap()
            .into_iter()
            .collect_vec();
        Self { fields }
    }
}

impl TicketConstraint {
    fn is_valid(&self, field_value: usize) -> bool {
        (self.start1..=self.end1).contains(&field_value)
            || (self.start2..=self.end2).contains(&field_value)
    }
}
