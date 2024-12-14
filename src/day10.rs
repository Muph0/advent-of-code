use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use crate::{utils::Grid2D, vec::Vec2i, Answer, Solution};

#[derive(Clone, Copy)]
pub struct Day10;

fn count_reachable_peaks(grid: &Grid2D<i32>, start: Vec2i) -> i64 {
    let mut seen = HashSet::<Vec2i>::new();
    let mut queue = VecDeque::<Vec2i>::new();

    queue.push_back(start);
    while let Some(pos) = queue.pop_front() {
        let h = grid[pos];
        for dir in Vec2i::directions_4() {
            let next = pos + dir;
            let Some(&next_h) = grid.get(next) else {
                continue;
            };

            if h + 1 == next_h && seen.insert(next) {
                queue.push_back(next);
            }
        }
    }
    seen.into_iter().filter(|x| grid[*x] == 9).count() as _
}

impl Solution for Day10 {
    fn day_number(&self) -> i32 {
        10
    }
    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(*self)
    }

    fn part_one(&self, input: &str) -> Answer {
        let mut sum = 0;
        let grid = Grid2D::from_str(input)
            .unwrap()
            .map(|c| c.to_digit(10).unwrap() as i32);

        for head in grid.iter_indices() {
            if grid[head] == 0 {
                sum += count_reachable_peaks(&grid, head.into());
            }
        }
        sum.into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let grid = Grid2D::from_str(input)
            .unwrap()
            .map(|c| c.to_digit(10).unwrap() as i32);

        let mut rating = grid.clone().map(|_| 0);
        let mut queue: VecDeque<Vec2i> = grid
            .iter_indices()
            .map(|p| p.into())
            .filter(|&p| grid[p] == 0)
            .collect();

        for pos in &queue {
            rating[*pos] = 1;
        }

        println!("grid:\n{}\nrating:\n{}", &grid, &rating);

        while let Some(pos) = queue.pop_front() {
            let h = grid[pos];
            //println!("pos = {pos}");

            for dir in Vec2i::directions_4() {
                let next = pos + dir;
                let Some(&next_h) = grid.get(next) else {
                    continue;
                };

                if h + 1 == next_h {
                    let mut r = 0;
                    for dir2 in Vec2i::directions_4() {
                        let prev = next + dir2;
                        let Some(&prev_h) = grid.get(prev) else {
                            continue;
                        };
                        if prev_h == h {
                            r += rating[prev];
                        }
                    }

                    if r > rating[next] {
                        rating[next] = r;
                        queue.push_back(next);
                    }
                }
            }
            //println!("rating:\n{}", &rating);
        }

        grid.iter_indices()
            .filter(|&p| grid[p] == 9)
            .map(|p| rating[p] as i64)
            .sum::<i64>()
            .into()
    }
}
