#![allow(unused)]

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashSet, VecDeque},
    mem::MaybeUninit,
    ops::{Deref, Index, IndexMut},
};

use multimap::MultiMap;

use crate::{utils::*, vec::Vec2i, Answer, Solution};

#[derive(Clone, Copy)]
pub struct Day05;

struct Order {
    before: MultiMap<i32, i32>,
}
impl Order {
    fn from_str(input: &str) -> Self {
        let mut before = MultiMap::new();
        //let mut after = MultiMap::new();

        for line in input.lines() {
            if line.trim().is_empty() {
                break;
            }
            let [a, b] = line
                .split("|")
                .map(|s| s.parse::<i32>().unwrap())
                .destruct();

            //after.insert(a, b);
            before.insert(b, a);
        }

        //make_transitive(&mut before);
        Self { before }
    }

    /// Is `a` before `b`
    fn is_before(&self, a: i32, b: i32) -> bool {
        a != b && self.before.contains(b, a)
    }

    fn to_transitive(&self) -> Self {
        let mut tr_before = MultiMap::new();

        Self { before: tr_before }
    }
}

impl Solution for Day05 {
    fn day_number(&self) -> i32 {
        5
    }
    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(*self)
    }

    fn part_one(&self, input: &str) -> Answer {
        let [order_str, prints] = input.split("\n\n").destruct();
        let order = Order::from_str(order_str);

        let mut sum = 0;
        'list: for list in prints.lines() {
            let list: Vec<i32> = list.split(",").map(|s| s.parse().unwrap()).collect();

            for i in 0..list.len() {
                for j in i..list.len() {
                    if i != j && order.is_before(list[j], list[i]) {
                        continue 'list;
                    }
                }
            }

            sum += list[list.len() / 2];
        }

        sum.into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let [order_str, prints] = input.split("\n\n").destruct();
        let order = Order::from_str(order_str);

        let mut sum = 0;
        for list_str in prints.lines() {
            let mut list: Vec<i32> = list_str.split(",").map(|s| s.parse().unwrap()).collect();

            let mut consider = false;
            let mut ordered = false;

            while !ordered {
                ordered = true;
                for i in 0..list.len() {
                    for j in i..list.len() {
                        if i != j && order.is_before(list[j], list[i]) {
                            consider = true;
                            ordered = false;
                            list.swap(i, j);
                        }
                    }
                }
            }

            if consider {
                sum += list[list.len() / 2];
            }
        }

        sum.into()
    }
}

fn make_transitive(map: &mut MultiMap<i32, i32>) {
    fn visit(x: i32, map: &mut MultiMap<i32, i32>, seen: &mut HashSet<i32>) {
        if seen.contains(&x) {
            return;
        }
        seen.insert(x);

        let mut ys: BTreeSet<_> = map.get_vec(&x).iter().map(|i| *i).collect();
        for y in &ys {
            visit(*y, map, seen);
        }

        for y in map.get_vec(&x).iter() {
            for z in map.get_vec(y).iter() {
                ys.insert(*z);
            }
        }

        if let Some(vec) = map.get_vec_mut(&x) {
            vec.clear();
            for y in ys {
                vec.push(y);
            }
        }
    }

    let rhss: HashSet<i32> = map.flat_iter().map(|(_, &v)| v).collect();
    let minimums: HashSet<i32> = map
        .keys()
        .filter(|k| !rhss.contains(k))
        .map(|i| *i)
        .collect();
    assert!(minimums.is_empty() == false);

    let mut seen = HashSet::new();
    for m in minimums {
        visit(m, map, &mut seen);
    }
}
