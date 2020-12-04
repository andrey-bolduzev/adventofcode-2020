#[macro_use]
extern crate maplit;
use aoc2020::parse;

use itertools::Itertools as _;
use parse_display::{Display, FromStr};
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

#[derive(Display, FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{field_type}:{value}")]
struct DocumentField {
    field_type: FieldType,
    value: String,
}

#[derive(Display, FromStr, Debug, PartialEq, Clone, Eq, Hash)]
#[allow(non_camel_case_types)]
enum FieldType {
    byr,
    iyr,
    eyr,
    hgt,
    hcl,
    ecl,
    pid,
    cid,
}

#[derive(Debug)]
struct TravelDoc {
    fields: HashSet<DocumentField>,
}

impl TravelDoc {
    fn is_valid_part1(&self) -> bool {
        let count = &self
            .fields
            .iter()
            .filter(|field| {
                field.field_type == FieldType::byr
                    || field.field_type == FieldType::iyr
                    || field.field_type == FieldType::eyr
                    || field.field_type == FieldType::hgt
                    || field.field_type == FieldType::hcl
                    || field.field_type == FieldType::ecl
                    || field.field_type == FieldType::pid
            })
            .count();

        count == &7
    }

    fn is_valid_part2(&self) -> bool {
        let hcl_re = Regex::new(r"#[0-9a-f]{6}").unwrap();
        let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
        let allowed_ecl = hashset! {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};

        let count = &self
            .fields
            .iter()
            .filter(|field| match field.field_type {
                FieldType::byr => (1920..=2002).contains(&field.value.parse::<usize>().unwrap()),
                FieldType::iyr => (2010..=2020).contains(&field.value.parse::<usize>().unwrap()),
                FieldType::eyr => (2020..=2030).contains(&field.value.parse::<usize>().unwrap()),
                FieldType::hgt if field.value.contains("cm") => {
                    (150..=193).contains(&field.value.replace("cm", "").parse::<usize>().unwrap())
                }
                FieldType::hgt if field.value.contains("in") => {
                    (59..=76).contains(&field.value.replace("in", "").parse::<usize>().unwrap())
                }
                FieldType::hcl => hcl_re.is_match(&field.value),
                FieldType::ecl => allowed_ecl.contains(&field.value.as_str()),
                FieldType::pid => pid_re.is_match(&field.value),
                _ => false,
            })
            .count();

        count == &7
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let docs = parse_docs(input);

    let filtered = docs
        .into_iter()
        .filter(TravelDoc::is_valid_part1)
        .collect_vec();

    let count = filtered.iter().count();
    println!("part 1 {}", count);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let docs = parse_docs(input);

    let filtered = docs
        .into_iter()
        .filter(TravelDoc::is_valid_part2)
        .collect_vec();

    let count = filtered.iter().count();
    println!("part 2 {}", count);

    Ok(())
}

fn parse_docs(input: &Path) -> Vec<TravelDoc> {
    use std::str::FromStr;

    let mut docs: Vec<TravelDoc> = Vec::new();

    let mut current_fields: HashSet<DocumentField> = HashSet::new();

    for line in parse::<String>(&input).unwrap() {
        if line.is_empty() {
            docs.push(TravelDoc {
                fields: current_fields.clone(),
            });
            current_fields.clear();
            continue;
        }

        let line_fields: Vec<DocumentField> = line
            .split(' ')
            .map(|item| DocumentField::from_str(&item).unwrap())
            .collect();

        current_fields.extend(line_fields);
    }

    // last line
    docs.push(TravelDoc {
        fields: current_fields,
    });
    docs
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
