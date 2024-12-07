use std::collections::HashSet;

use crate::{utils::Grid2D, vec::Vec2i, Answer, Solution};

#[derive(Debug, Clone)]
pub struct Day06;

#[derive(Clone, Copy)]
struct Guard {
    pos: Vec2i,
    dir: Vec2i,
}

fn get_guard_start(grid: &Grid2D<char>) -> Guard {
    let pos: Vec2i = grid.index_of(|c| "^v><".contains(*c)).unwrap().into();
    let dir: Vec2i = match grid[pos] {
        '^' => (0, -1).into(),
        'v' => (0, 1).into(),
        '<' => (-1, 0).into(),
        '>' => (1, 0).into(),
        _ => panic!(),
    };
    Guard { pos, dir }
}

fn walk_guard(mut guard: Guard, grid: &Grid2D<char>) -> (HashSet<Vec2i>, bool) {
    let mut circular = false;
    let mut visited = HashSet::new();
    while let Some(_) = grid.get(guard.pos) {
        let in_front = guard.pos + guard.dir;
        if visited.insert((guard.pos, guard.dir)) == false {
            circular = true;
            break;
        }

        match grid.get(in_front) {
            Some('#') => guard.dir = guard.dir.rot_x2y(),
            _ => guard.pos = in_front,
        }
    }

    (visited.iter().map(|(pos, _)| *pos).collect(), circular)
}

impl Solution for Day06 {
    fn day_number(&self) -> i32 {
        6
    }
    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(self.clone())
    }

    fn part_one(&self, input: &str) -> Answer {
        let grid = Grid2D::from_str(input);
        let guard = get_guard_start(&grid);
        let (visited, _) = walk_guard(guard, &grid);
        visited.len().into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let mut grid = Grid2D::from_str(input);
        let guard = get_guard_start(&grid);
        let (visited, _) = walk_guard(guard, &grid);
        let mut sum = 0;

        for v in visited {
            let old = grid[v];

            grid[v] = '#';
            let (_, circular) = walk_guard(guard, &grid);
            if circular {
                sum += 1;
            }
            grid[v] = old;
        }

        sum.into()
    }
}
