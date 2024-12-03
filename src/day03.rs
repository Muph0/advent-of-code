use regex::Regex;

use crate::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn day_number(&self) -> i32 {
        3
    }

    fn part_one(&self, input: &str) -> String {
        let mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut sum = 0;
        for cap in mul.captures_iter(input) {
            let g = cap.extract::<2>().1;
            let a: i32 = g[0].parse().unwrap();
            let b: i32 = g[1].parse().unwrap();

            sum += a * b;
        }

        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mul = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

        let mut sum = 0;
        let mut enable = true;
        for cap in mul.captures_iter(input) {
            let ident = cap.get(0).unwrap().as_str().split('(').next().unwrap();

            match ident {
                "do" => {
                    enable = true;
                }
                "don't" => {
                    enable = false;
                }
                "mul" if enable => {
                    let a: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
                    let b: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
                    sum += a * b;
                }
                _ => (),
            }
        }

        sum.to_string()
    }
}
