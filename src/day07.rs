use std::str::FromStr;

use crate::{utils::DestructIterator, Answer, Solution};

#[derive(Clone, Copy)]
pub struct Day07;

enum Operator {
    Add,
    Mul,
    Concat,
}

struct Challenge {
    target: i64,
    operands: Vec<i64>,
    orders: Vec<i64>,
}
impl FromStr for Challenge {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [result_str, ops_str] = s.split(':').destruct();

        let expected: i64 = result_str.parse().unwrap();
        let operands: Vec<i64> = ops_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let orders = ops_str
            .split_whitespace()
            .map(|s| 10i64.pow(s.len() as _))
            .collect();

        Ok(Self {
            target: expected,
            operands,
            orders,
        })
    }
}

impl Solution for Day07 {
    fn day_number(&self) -> i32 {
        7
    }

    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(*self)
    }

    fn part_one(&self, input: &str) -> Answer {
        let ops = [
            //
            Operator::Mul,
            Operator::Add,
        ];
        compute_target_sum(input, &ops).into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let ops = [
            //
            Operator::Concat,
            Operator::Mul,
            Operator::Add,
        ];
        compute_target_sum(input, &ops).into()
    }
}

fn compute_target_sum(input: &str, ops: &[Operator]) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let ch: Challenge = line.parse().unwrap();
        if try_operators(0, ch.target, &ch.operands, &ch.orders, ops) {
            sum += ch.target;
        }
    }
    sum.into()
}

fn try_operators(current: i64, target: i64, suffix: &[i64], ord: &[i64], ops: &[Operator]) -> bool {
    if suffix.len() == 0 {
        return current == target;
    }
    if current > target {
        return false;
    }

    for op in ops {
        let next = match op {
            Operator::Add => current + suffix[0],
            Operator::Mul => current * suffix[0],
            Operator::Concat => current * ord[0] + suffix[0],
        };

        if try_operators(next, target, &suffix[1..], &ord[1..], ops) {
            return true;
        }
    }

    false
}
