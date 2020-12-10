use aoc2020::parse;

use color_eyre::{eyre::eyre, Result};
use itertools::Itertools as _;
use std::collections::HashMap;
use std::path::Path;

pub fn part1(input: &Path) -> Result<()> {
    let all_adapters = collect_sorted_adapters(&input)?;

    let mut ones = 0;
    let mut threes = 0;

    for (a, b) in all_adapters.iter().tuple_windows() {
        match b - a {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }

    println!("part 1 {}", ones * threes);

    assert_eq!(1980, ones * threes);

    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    let all_adapters = collect_sorted_adapters(&input)?;

    let mut cache = HashMap::new();

    let count = count_for(
        0,
        &all_adapters,
        *all_adapters.last().ok_or_else(|| eyre!("not found"))?,
        &mut cache,
    );

    println!("part 2 {}", count);

    assert_eq!(4628074479616, count);

    Ok(())
}

fn collect_sorted_adapters(input: &Path) -> Result<Vec<usize>> {
    let mut all_adapters = parse::<usize>(&input)?.sorted().collect_vec();
    let last = all_adapters.iter().max().unwrap() + 3;

    all_adapters.insert(0, 0);
    all_adapters.push(last);
    Ok(all_adapters)
}

fn count_for(
    origin: usize,
    adapters: &[usize],
    max: usize,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if origin == max {
        return 1;
    }

    if let Some(&count) = cache.get(&origin) {
        return count;
    }

    let mut branches = 0;

    if adapters.contains(&(origin + 1)) {
        branches += count_for(origin + 1, adapters, max, cache);
    }
    if adapters.contains(&(origin + 2)) {
        branches += count_for(origin + 2, adapters, max, cache);
    }
    if adapters.contains(&(origin + 3)) {
        branches += count_for(origin + 3, adapters, max, cache);
    }
    cache.insert(origin, branches);
    branches
}
