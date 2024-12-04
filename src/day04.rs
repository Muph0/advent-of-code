use std::ops::{Index, IndexMut};

use crate::{vec::Vec2i, Solution};

pub struct Day04;

struct Board {
    width: i32,
    height: i32,
    data: Vec<char>,
}
impl Board {
    fn from_str(input: &str) -> Self {
        let lines = input.lines();

        let width = lines.clone().next().unwrap().len() as i32;
        let height = lines.clone().filter(|l| l.trim().len() != 0).count() as i32;
        let mut data = Vec::new();
        data.reserve((width * height) as _);

        for l in lines {
            if data.len() as i32 == width * height {
                break;
            }
            assert_eq!(l.len(), width as usize);

            for c in l.chars() {
                data.push(c);
            }
        }
        Self {
            width,
            height,
            data,
        }
    }

    fn word_count(&self, word: &str) -> i32 {
        let mut count = 0;

        for sq in self.squares() {
            'dirs: for dir in Vec2i::directions_8() {
                for (i, expect_ch) in word.chars().enumerate() {
                    let next_ch = match self.get(sq + dir * i as i32) {
                        None => continue 'dirs,
                        Some(c) => *c,
                    };

                    if next_ch != expect_ch {
                        continue 'dirs;
                    }
                }

                // all letters were equal at this point
                count += 1;
            }
        }
        count
    }

    fn x_shape_mas_count(&self) -> i32 {
        let mut count = 0;

        for sq in self.squares() {
            let x_edge = sq.x == 0 || sq.x == self.width - 1;
            let y_edge = sq.y == 0 || sq.y == self.height - 1;

            if x_edge || y_edge || self[sq] != 'A' {
                continue;
            }
            for mut dir in Vec2i::directions_4_diag() {
                if self[sq + dir] != 'M' {
                    continue;
                }
                dir = dir.right90();
                if self[sq + dir] != 'M' {
                    continue;
                }
                dir = dir.right90();
                if self[sq + dir] != 'S' {
                    continue;
                }
                dir = dir.right90();
                if self[sq + dir] != 'S' {
                    continue;
                }
                count += 1;
            }
        }
        count
    }
}

impl Solution for Day04 {
    fn day_number(&self) -> i32 {
        4
    }

    fn part_one(&self, input: &str) -> String {
        let board = Board::from_str(input);
        board.word_count("XMAS").to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let board = Board::from_str(input);
        board.x_shape_mas_count().to_string()
    }
}

impl Index<Vec2i> for Board {
    type Output = char;
    fn index(&self, index: Vec2i) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl IndexMut<Vec2i> for Board {
    fn index_mut(&mut self, index: Vec2i) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl Board {
    fn squares(&self) -> impl Iterator<Item = Vec2i> {
        let w = self.width;
        (0..self.data.len() as i32)
            .into_iter()
            .map(move |i| (i % w, i / w).into())
    }

    pub fn get(&self, index: Vec2i) -> Option<&char> {
        let (x, y) = index.into();
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(&self.data[(x + y * self.width) as usize])
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, index: Vec2i) -> Option<&mut char> {
        let (x, y) = index.into();
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(&mut self.data[(x + y * self.width) as usize])
        } else {
            None
        }
    }
}
