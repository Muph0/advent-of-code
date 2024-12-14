use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::Add,
    str::FromStr,
};

use crate::{utils::Grid2D, vec::Vec2i, Solution};

#[derive(Clone, Copy)]
pub struct Day12;

impl Solution for Day12 {
    fn day_number(&self) -> i32 {
        12
    }

    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(*self)
    }

    fn part_one(&self, input: &str) -> crate::Answer {
        let grid = Grid2D::from_str(input).unwrap();
        let mut sum = 0;

        let mut used = HashSet::<Vec2i>::new();
        for pos in grid.iter_vec2() {
            if used.contains(&pos) {
                continue;
            }

            let plot = find_plot_edges(&grid, pos, &mut used);
            sum += plot.area * plot.perimeter;
        }

        sum.into()
    }

    fn part_two(&self, input: &str) -> crate::Answer {
        let grid = Grid2D::from_str(input).unwrap();
        let mut sum = 0;

        let mut used = HashSet::<Vec2i>::new();
        for pos in grid.iter_vec2() {
            if used.contains(&pos) {
                continue;
            }

            let plot = find_plot_corners(&grid, pos, &mut used);
            sum += plot.area * plot.perimeter;
        }

        sum.into()
    }
}

struct Plot {
    area: i32,
    perimeter: i32,
}
fn find_plot_edges(grid: &Grid2D<char>, start: Vec2i, used: &mut HashSet<Vec2i>) -> Plot {
    let mut perimeter = 0;
    let mut area = 0;
    let ch = grid[start];

    let mut queue = VecDeque::from([start]);
    while let Some(pos) = queue.pop_front() {
        if used.contains(&pos) {
            continue;
        }

        area += 1;
        used.insert(pos);

        for dir in Vec2i::directions_4() {
            if grid.get(pos + dir) != Some(&ch) {
                perimeter += 1;
            } else {
                queue.push_back(pos + dir);
            }
        }
    }

    Plot { area, perimeter }
}

fn find_plot_corners(grid: &Grid2D<char>, start: Vec2i, used: &mut HashSet<Vec2i>) -> Plot {
    let mut corners = 0;
    let mut area = 0;
    let ch = grid[start];

    let mut queue = VecDeque::from([start]);
    while let Some(pos) = queue.pop_front() {
        if used.contains(&pos) {
            continue;
        }

        area += 1;
        used.insert(pos);

        for dir in Vec2i::directions_4() {
            let dir_r = dir.rot_x2y();
            let same_char = |d: Vec2i| grid.get(pos + d) == Some(&ch);

            if !same_char(dir) {
                if !same_char(dir_r) {
                    corners += 1;
                }
            } else {
                queue.push_back(pos + dir);

                if same_char(dir_r) && !same_char(dir + dir_r) {
                    corners += 1;
                }
            }
        }
    }

    Plot {
        area,
        perimeter: corners,
    }
}
