use aoc2020::parse;

use itertools::Itertools as _;
use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let cube_lines = parse::<String>(&input)?.collect_vec();
    let mut dimension = Dimension::new(cube_lines);
    let mut cycle = 0;
    let mut low_bound = 0;
    let mut high_bound = 7;

    while cycle <= 5 {
        cycle += 1;
        high_bound += 1;
        low_bound -= 1;
        let high_z = cycle;
        let low_z = -cycle;

        let snapshot = dimension.clone();

        for row in low_bound..=high_bound {
            for column in low_bound..=high_bound {
                for z in low_z..=high_z {
                    for w in 0..=0 {
                        let candidate = Cube {
                            x: column,
                            y: row,
                            z,
                            w,
                        };
                        if candidate.new_state(&snapshot) {
                            dimension.active_cubes.insert(candidate);
                        } else {
                            dimension.active_cubes.remove(&candidate);
                        }
                    }
                }
            }
        }
    }

    let answer = dimension.count_active();

    println!("part 1 {}", answer);

    assert_eq!(257, answer);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let cube_lines = parse::<String>(&input)?.collect_vec();
    let mut dimension = Dimension::new(cube_lines);
    let mut cycle = 0;
    let mut low_bound = 0;
    let mut high_bound = 7;

    while cycle <= 5 {
        cycle += 1;
        high_bound += 1;
        low_bound -= 1;
        let high_z = cycle;
        let low_z = -cycle;

        let snapshot = dimension.clone();

        for row in low_bound..=high_bound {
            for column in low_bound..=high_bound {
                for z in low_z..=high_z {
                    for w in low_z..=high_z {
                        let candidate = Cube {
                            x: column,
                            y: row,
                            z,
                            w,
                        };
                        if candidate.new_state(&snapshot) {
                            dimension.active_cubes.insert(candidate);
                        } else {
                            dimension.active_cubes.remove(&candidate);
                        }
                    }
                }
            }
        }
    }

    let answer = dimension.count_active();

    println!("part 2 {}", answer);

    assert_eq!(2532, answer);

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

#[derive(Debug, Clone)]
struct Dimension {
    active_cubes: HashSet<Cube>,
}

impl Cube {
    fn new_state(&self, dimension: &Dimension) -> bool {
        let active_neighbors = self
            .generate_neighbours()
            .iter()
            .filter(|n| dimension.active_cubes.contains(n))
            .count();

        let is_active = dimension.active_cubes.contains(self);

        is_active && (2..=3).contains(&active_neighbors) || !is_active && active_neighbors == 3
    }

    fn generate_neighbours(&self) -> HashSet<Self> {
        let mut neighbors = HashSet::new();

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        neighbors.insert(Cube {
                            x: self.x + x,
                            y: self.y + y,
                            z: self.z + z,
                            w: self.w + w,
                        });
                    }
                }
            }
        }

        neighbors.remove(self);

        neighbors
    }
}

impl Dimension {
    fn new(cube_lines: Vec<String>) -> Self {
        let mut active_cubes = HashSet::new();

        for (row, line) in cube_lines.iter().enumerate() {
            for (column, c) in line.chars().enumerate() {
                if c == '#' {
                    active_cubes.insert(Cube {
                        x: column as isize,
                        y: row as isize,
                        z: 0,
                        w: 0,
                    });
                }
            }
        }

        Self { active_cubes }
    }

    fn count_active(&self) -> usize {
        self.active_cubes.len()
    }
}
