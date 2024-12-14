use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Add,
};

use crate::Solution;

#[derive(Clone, Copy)]
pub struct Day11;

impl Solution for Day11 {
    fn day_number(&self) -> i32 {
        11
    }

    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(*self)
    }

    fn part_one(&self, input: &str) -> crate::Answer {
        let mut stones: Vec<i64> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for i in 0..25 {
            println!("blink {i}, {} stones", stones.len());
            let mut new_stones = Vec::new();

            for k in &stones {
                //let log10 = f64::log10(*k as f64).round() as i64;
                let kstr = k.to_string();
                if *k == 0 {
                    new_stones.push(1);
                //} else if log10 % 2 == 0 {
                //    let half_digits = 10i64.pow(log10 as u32 / 2);
                //    let (a, b) = (k / half_digits, k % half_digits);
                //    new_stones.push(a);
                //    new_stones.push(b);
                } else if kstr.len() % 2 == 0 {
                    let half = kstr.len() / 2;
                    let a: i64 = kstr[0..half].parse().unwrap();
                    let b: i64 = kstr[half..].parse().unwrap();
                    new_stones.push(a);
                    new_stones.push(b);
                } else {
                    new_stones.push(k * 2024);
                }
            }
            stones.clear();
            stones.append(&mut new_stones);
            //println!("{:?}", &stones);
        }

        let unique: HashSet<_> = stones.iter().collect();
        println!("{} unique stones", unique.len());

        stones.len().into()
    }

    fn part_two(&self, input: &str) -> crate::Answer {
        let mut stones: HashMap<i64, i64> = input
            .split_whitespace()
            .map(|s| (s.parse().unwrap(), 1))
            .collect();

        for i in 0..75 {
            println!("blink {}, {} stones", i + 1, stones.values().sum::<i64>());
            let mut new_stones = HashMap::new();

            for (k, amount) in &stones {
                //let log10 = f64::log10(*k as f64).round() as i64;
                let kstr = k.to_string();
                if *k == 0 {
                    new_stones.add(1, *amount);
                //} else if log10 % 2 == 0 {
                //    let half_digits = 10i64.pow(log10 as u32 / 2);
                //    let (a, b) = (k / half_digits, k % half_digits);
                //    new_stones.add(a, *amount);
                //    new_stones.add(b, *amount);
                } else if kstr.len() % 2 == 0 {
                    let half = kstr.len() / 2;
                    let a: i64 = kstr[0..half].parse().unwrap();
                    let b: i64 = kstr[half..].parse().unwrap();
                    new_stones.add(a, *amount);
                    new_stones.add(b, *amount);
                } else {
                    new_stones.add(k * 2024, *amount);
                }
            }
            stones = new_stones;
            //println!("{:?}", &stones);
        }

        stones.values().sum::<i64>().into()
    }
}

trait HashMapAdd<K, V: Add> {
    fn add(&mut self, k: K, v: V);
}

impl<K: Eq + Hash, V: Clone + Add<V, Output = V>> HashMapAdd<K, V> for HashMap<K, V> {
    fn add(&mut self, k: K, v: V) {
        if let Some(existing_v) = self.get_mut(&k) {
            *existing_v = existing_v.clone() + v;
        } else {
            self.insert(k, v);
        }
    }
}
