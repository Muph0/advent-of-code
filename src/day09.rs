use multimap::MultiMap;

use crate::utils::*;
use crate::{vec::Vec2i, Answer, Solution};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, write};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Clone, Copy)]
pub struct Day09;

#[derive(Clone)]
struct Disk {
    blocks: Vec<Option<u32>>,
}
#[derive(Debug, Clone, Copy)]
struct File {
    id: u32,
    start: usize,
    len: usize,
}
impl File {
    fn to_gap(&self) -> Gap {
        Gap::new(self.len, self.start)
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Gap {
    len: usize,
    start: usize,
}
impl Gap {
    fn new(len: usize, start: usize) -> Self {
        Self { len, start }
    }
    fn merge(mut self, other: Gap) -> Gap {
        if self.start > other.start {
            return other.merge(self);
        }
        assert_eq!(self.start + self.len, other.start);
        self.len += other.len;
        self
    }

    fn offset_start(&self, amt: usize) -> Gap {
        Gap {
            len: self.len - amt,
            start: self.start + amt,
        }
    }
}
impl fmt::Debug for Gap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.start + self.len)
    }
}

impl Disk {
    fn from_disk_map(input: &str) -> Self {
        let mut blocks = Vec::new();

        for (i, c) in input.chars().enumerate() {
            let is_file = i % 2 == 0;
            let file_id = i as u32 / 2;
            let len = c.to_digit(10).unwrap();

            let val = match is_file {
                true => Some(file_id),
                false => None,
            };

            for _ in 0..len {
                blocks.push(val);
            }
        }

        Self { blocks }
    }

    fn checksum(&self) -> u64 {
        let mut checksum = 0;
        for (i, id) in self.blocks.iter().enumerate() {
            if let Some(id) = id {
                checksum += (i as u64) * (*id as u64);
            }
        }
        checksum
    }

    fn compact(&mut self) {
        for i in 0.. {
            if i >= self.blocks.len() {
                break;
            }
            if self.blocks[i].is_none() {
                let moved_id = loop {
                    let last = self.blocks.pop().unwrap();
                    if let Some(last) = last {
                        break last;
                    }
                };
                self.blocks[i] = Some(moved_id);
            }
        }
    }

    fn compact2(&mut self) {
        let mut gaps = BTreeSet::new();
        let mut files = Vec::new();
        let mut cur_gap = 0;
        let mut cur_file_len = 0;
        let mut prev_id = Some(0);

        for (i, id) in self.blocks.iter().chain([None].iter()).enumerate() {
            if *id != prev_id && prev_id.is_some() {
                files.push(File {
                    id: prev_id.unwrap(),
                    start: i - cur_file_len,
                    len: cur_file_len,
                });
                cur_file_len = 0;
            }

            if id.is_none() {
                cur_gap += 1;
            } else {
                cur_file_len += 1;
                if cur_gap != 0 {
                    gaps.insert(Gap::new(cur_gap, i - cur_gap));
                    cur_gap = 0;
                }
            }

            prev_id = *id;
        }

        for file in files.iter().rev() {
            //println!("file {file:?}");

            // find smallest gaps that fit
            let first_fit = gaps.range(Gap::new(file.len, 0)..);

            // take the first one
            match first_fit.filter(|g| g.start < file.start).next() {
                Some(&gap) => {
                    //println!("move to (len, start)={gap:?}");
                    let ok = gaps.remove(&gap);
                    assert!(ok);

                    // moving file creates a gap
                    let mut new_gap = file.to_gap();
                    if let Some(gap_before) = self.find_gap(file.start - 1) {
                        let ok = gaps.remove(&gap_before);
                        assert!(ok);
                        new_gap = new_gap.merge(gap_before);
                    }
                    if let Some(gap_after) = self.find_gap(file.start + file.len) {
                        let ok = gaps.remove(&gap_after);
                        assert!(ok);
                        new_gap = new_gap.merge(gap_after);
                    }
                    gaps.insert(new_gap);

                    for i in 0..file.len {
                        let moved = self.blocks[file.start + i];
                        assert_eq!(moved, Some(file.id));
                        self.blocks[file.start + i] = None;
                        self.blocks[gap.start + i] = moved;
                    }

                    if file.len < gap.len {
                        // shrink the used gap
                        gaps.insert(gap.offset_start(file.len));
                    }
                    //println!("moved: {}", self);
                    //println!("gaps: {gaps:?}");
                }
                _ => (),
            }
        }
    }

    fn find_gap(&self, i: usize) -> Option<Gap> {
        match self.blocks.get(i) {
            Some(Some(_)) => return None,
            None => return None,
            _ => (),
        }

        let mut start = i;
        while start > 0 && self.blocks[start - 1].is_none() {
            start -= 1;
        }
        let mut end = i;
        while end <= self.blocks.len() - 1 && self.blocks[end].is_none() {
            end += 1;
        }

        Some(Gap::new(end - start, start))
    }
}
impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for b in self.blocks.iter() {
            match b {
                Some(i) => list.entry(i),
                None => list.entry(&"_"),
            };
        }
        list.finish()
    }
}

impl Solution for Day09 {
    fn day_number(&self) -> i32 {
        9
    }

    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(*self)
    }

    fn part_one(&self, input: &str) -> Answer {
        let mut disk = Disk::from_disk_map(input);
        disk.compact();
        let checksum = disk.checksum();

        //println!("{}", &disk);
        checksum.into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let mut disk = Disk::from_disk_map(input);
        //println!("Before\n{}", &disk);
        disk.compact2();
        let checksum = disk.checksum();

        //println!("After\n{}", &disk);
        checksum.into()
    }
}
