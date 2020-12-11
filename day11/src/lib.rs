use aoc2020::parse;

use grid::*;
use itertools::Itertools as _;
use std::cmp::*;
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let lines: Vec<Vec<char>> = parse::<String>(&input)?
        .map(|string| string.chars().collect_vec())
        .collect_vec();

    let mut grid = grid![];

    for line in lines.into_iter() {
        grid.push_row(line);
    }

    let mut starting_grid = grid;

    loop {
        let new_grid = move_around_part1(starting_grid.clone());

        if starting_grid == new_grid {
            break;
        }

        starting_grid = new_grid;
    }

    let occupied = starting_grid.iter().filter(|&&cell| cell == '#').count();
    println!("part 1 {}", occupied);

    assert_eq!(2319, occupied);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let lines: Vec<Vec<char>> = parse::<String>(&input)?
        .map(|string| string.chars().collect_vec())
        .collect_vec();

    let mut grid = grid![];

    for line in lines.into_iter() {
        grid.push_row(line);
    }

    let mut starting_grid = grid;

    loop {
        let new_grid = move_around_part2(starting_grid.clone());

        if starting_grid == new_grid {
            break;
        }

        starting_grid = new_grid;
    }

    let occupied = starting_grid.iter().filter(|&&cell| cell == '#').count();
    println!("part 2 {}", occupied);

    assert_eq!(2117, occupied);

    Ok(())
}

fn move_around_part1(mut grid: Grid<char>) -> Grid<char> {
    let (rows, cols) = grid.size();

    let cloned_grid = ConvenientGrid {
        inner: grid.clone(),
    };
    for row in 0..rows {
        for col in 0..cols {
            let cell = grid.get_mut(row, col).unwrap();

            match cell {
                'L' if check_empty_neighbours_part1(row as i32, col as i32, &cloned_grid) => {
                    *cell = '#'
                }
                '#' if check_four_adjacent_part1(row as i32, col as i32, &cloned_grid) => {
                    *cell = 'L'
                }
                _ => {}
            }
        }
    }
    grid
}

fn check_empty_neighbours_part1(row: i32, col: i32, grid: &ConvenientGrid<char>) -> bool {
    grid.neighbors(row, col).iter().all(|n| **n != '#')
}

fn check_four_adjacent_part1(row: i32, col: i32, grid: &ConvenientGrid<char>) -> bool {
    grid.neighbors(row, col)
        .iter()
        .filter(|n| ***n == '#')
        .count()
        >= 4
}

struct ConvenientGrid<T> {
    inner: Grid<T>,
}

impl<T: Clone> ConvenientGrid<T> {
    fn get(&self, row: i32, col: i32) -> Option<&T> {
        if row < 0 || col < 0 {
            return None;
        }

        self.inner.get(row as usize, col as usize)
    }

    fn neighbors(&self, row: i32, col: i32) -> Vec<&T> {
        let mut neighbors = vec![];

        for i in row - 1..=row + 1 {
            for j in col - 1..=col + 1 {
                if i == row && j == col {
                    continue;
                }

                if let Some(neighbor) = self.get(i, j) {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors
    }
}

impl ConvenientGrid<char> {
    fn find_neighbors(&self, row: i32, col: i32) -> Vec<&char> {
        let mut neighbors = vec![];

        let mut current_row = row;
        let mut current_col = col;

        loop {
            current_row -= 1;
            if let Some(candidate) = self.get(current_row, current_col) {
                if *candidate != '.' {
                    neighbors.push(candidate);
                    break;
                }
            } else {
                break;
            }
        }

        current_row = row;
        current_col = col;

        loop {
            current_row += 1;
            if let Some(candidate) = self.get(current_row, current_col) {
                if *candidate != '.' {
                    neighbors.push(candidate);
                    break;
                }
            } else {
                break;
            }
        }

        current_row = row;
        current_col = col;

        loop {
            current_col -= 1;
            if let Some(candidate) = self.get(current_row, current_col) {
                if *candidate != '.' {
                    neighbors.push(candidate);
                    break;
                }
            } else {
                break;
            }
        }

        current_row = row;
        current_col = col;

        loop {
            current_col += 1;
            if let Some(candidate) = self.get(current_row, current_col) {
                if *candidate != '.' {
                    neighbors.push(candidate);
                    break;
                }
            } else {
                break;
            }
        }

        current_row = row;
        current_col = col;

        loop {
            current_row -= 1;
            current_col -= 1;
            if let Some(candidate) = self.get(current_row, current_col) {
                if *candidate != '.' {
                    neighbors.push(candidate);
                    break;
                }
            } else {
                break;
            }
        }

        current_row = row;
        current_col = col;

        loop {
            current_row += 1;
            current_col -= 1;
            if let Some(candidate) = self.get(current_row, current_col) {
                if *candidate != '.' {
                    neighbors.push(candidate);
                    break;
                }
            } else {
                break;
            }
        }

        current_row = row;
        current_col = col;

        loop {
            current_row -= 1;
            current_col += 1;
            if let Some(candidate) = self.get(current_row, current_col) {
                if *candidate != '.' {
                    neighbors.push(candidate);
                    break;
                }
            } else {
                break;
            }
        }

        current_row = row;
        current_col = col;

        loop {
            current_row += 1;
            current_col += 1;
            if let Some(candidate) = self.get(current_row, current_col) {
                if *candidate != '.' {
                    neighbors.push(candidate);
                    break;
                }
            } else {
                break;
            }
        }

        neighbors
    }
}

fn move_around_part2(mut grid: Grid<char>) -> Grid<char> {
    let (rows, cols) = grid.size();

    let cloned_grid = ConvenientGrid {
        inner: grid.clone(),
    };
    for row in 0..rows {
        for col in 0..cols {
            let cell = grid.get_mut(row, col).unwrap();

            match cell {
                'L' if check_empty_neighbours_part2(row as i32, col as i32, &cloned_grid) => {
                    *cell = '#'
                }
                '#' if check_five_adjacent_part2(row as i32, col as i32, &cloned_grid) => {
                    *cell = 'L'
                }
                _ => {}
            }
        }
    }
    grid
}

fn check_empty_neighbours_part2(row: i32, col: i32, grid: &ConvenientGrid<char>) -> bool {
    grid.find_neighbors(row, col).iter().all(|n| **n != '#')
}

fn check_five_adjacent_part2(row: i32, col: i32, grid: &ConvenientGrid<char>) -> bool {
    grid.find_neighbors(row, col)
        .iter()
        .filter(|n| ***n == '#')
        .count()
        >= 5
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
