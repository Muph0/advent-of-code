use std::{str::FromStr, thread};

use crate::{utils::DestructIterator, Answer, Solution};

#[derive(Clone, Copy)]
pub struct Day07;

enum Operator {
    Add,
    Mul,
    Cat,
}


impl Solution for Day07 {
    fn day_number(&self) -> i32 {
        7
    }

    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(*self)
    }

    fn part_one(&self, input: &str) -> Answer {
        compute_result_sum(input, |op_cnt| 2i64.pow(op_cnt as u32), to_bin_ops)
    }

    fn part_two(&self, input: &str) -> Answer {
        compute_result_sum(input, |op_cnt| 3i64.pow(op_cnt as u32), to_tern_ops)
    }
}

fn to_bin_ops(mut i: i64, amt: i64) -> impl Iterator<Item = Operator> {
    (0..amt).into_iter().map(move |_| {
        let ii = i;
        i /= 2;
        match ii % 2 {
            0 => Operator::Add,
            1 => Operator::Mul,
            _ => panic!(),
        }
    })
}

fn to_tern_ops(mut i: i64, amt: i64) -> impl Iterator<Item = Operator> {
    (0..amt).into_iter().map(move |_| {
        let ii = i;
        i /= 3;
        match ii % 3 {
            0 => Operator::Add,
            1 => Operator::Mul,
            2 => Operator::Cat,
            _ => panic!(),
        }
    })
}
fn compute_result_sum<I>(
    input: &str,
    option_count: fn(i64) -> i64,
    operator_iter: fn(i64, i64) -> I,
) -> Answer
where
    I: Iterator<Item = Operator>,
{
    let mut sum: i64 = 0;

    for line in input.lines() {
        let [result_str, ops_str] = line.split(':').destruct();

        let expected: i64 = result_str.parse().unwrap();
        let operands: Vec<i64> = ops_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let operator_order: Vec<i64> = ops_str
            .split_whitespace()
            .map(|s| 10i64.pow(s.len() as _))
            .collect();

        let operator_cnt = operands.len() as i64 - 1;

        for i in 0..option_count(operator_cnt) {
            let mut result = operands[0];

            let bits = operator_iter(i, operator_cnt);
            for ((opi, x), op) in operands.iter().enumerate().skip(1).zip(bits) {
                result = match op {
                    Operator::Add => result + x,
                    Operator::Mul => result * x,
                    Operator::Cat => result * operator_order[opi] + x,
                };
            }

            if result == expected {
                sum += expected;
                break;
            }
        }
    }

    sum.into()
}
