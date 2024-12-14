use crate::{Answer, Solution};
use std::{collections::BTreeSet, fmt};

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
        let file_ids: BTreeSet<_> = self.blocks.iter().filter_map(|x| *x).collect();

        for id in file_ids.iter().rev() {
            let file = self.find_file(*id);
            let Some(gap) = self.first_fit(file.len) else {
                continue;
            };
            if gap.start > file.start {
                continue;
            }

            for i in 0..file.len {
                self.blocks[file.start + i] = None;
                self.blocks[gap.start + i] = Some(*id);
            }
        }
    }

    fn find_file(&self, id: u32) -> File {
        let it = self.blocks.iter().enumerate();
        let start = it.clone().find(|x| *x.1 == Some(id)).unwrap().0;
        let end = it
            .skip(start)
            .find(|x| *x.1 != Some(id))
            .unwrap_or((self.blocks.len(), &None))
            .0;
        File {
            id,
            start,
            len: end - start,
        }
    }
    fn first_fit(&self, size: usize) -> Option<Gap> {
        let mut len = 0;
        for (i, id) in self.blocks.iter().enumerate() {
            if id.is_none() {
                len += 1;
            } else {
                if len >= size {
                    return Some(Gap {
                        start: i - len,
                        len,
                    });
                }
                len = 0;
            }
        }
        None
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
