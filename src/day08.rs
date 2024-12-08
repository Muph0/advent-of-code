use multimap::MultiMap;

use crate::utils::*;
use crate::{vec::Vec2i, Answer, Solution};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Clone, Copy)]
pub struct Day08;

struct Antennas {
    by_type: MultiMap<char, Vec2i>,
}
impl From<&Grid2D<char>> for Antennas {
    fn from(grid: &Grid2D<char>) -> Self {
        let mut by_type = MultiMap::new();
        for pos in grid.iter_indices() {
            let c = grid[pos];
            if c != '.' {
                by_type.insert(c, Vec2i::from(pos));
            }
        }
        Self { by_type }
    }
}

impl Solution for Day08 {
    fn day_number(&self) -> i32 {
        8
    }

    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(*self)
    }

    fn part_one(&self, input: &str) -> Answer {
        let grid: Grid2D<_> = input.parse().unwrap();
        let antennas = Antennas::from(&grid);

        let mut antinodes = HashSet::new();
        for (_c, nodes) in antennas.by_type {
            for (a, b) in nodes.iter().all_pairs() {
                let directions = [(a, b), (b, a)];
                for (a, b) in directions {
                    let dir = *b - *a;
                    let node = *b + dir;
                    if grid.get(node).is_some() {
                        antinodes.insert(node);
                    }
                }
            }
        }
        antinodes.len().into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let grid: Grid2D<_> = input.parse().unwrap();
        let antennas = Antennas::from(&grid);

        let mut antinodes = HashSet::new();
        for (_c, nodes) in antennas.by_type {
            for (a, b) in nodes.iter().all_pairs() {
                let directions = [(a, b), (b, a)];
                for (a, b) in directions {
                    let dir = *a - *b;
                    let mut node = *b + dir;
                    while grid.get(node).is_some() {
                        antinodes.insert(node);
                        node = node + dir;
                    }
                }
            }
        }
        antinodes.len().into()
    }
}
