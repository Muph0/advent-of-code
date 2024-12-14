use regex::Regex;

use crate::{Answer, Solution};

#[derive(Clone, Copy)]
pub struct Day03;

impl Solution for Day03 {
    fn day_number(&self) -> i32 {
        3
    }
    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(*self)
    }

    fn part_one(&self, input: &str) -> Answer {
        let mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut sum = 0;
        for cap in mul.captures_iter(input) {
            let g = cap.extract::<2>().1;
            let a: i32 = g[0].parse().unwrap();
            let b: i32 = g[1].parse().unwrap();

            sum += a * b;
        }

        sum.into()
    }

    #[allow(unreachable_code)]
    fn part_two(&self, input: &str) -> Answer {
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

        let sum2 = part_two_2(input);
        assert_eq!(sum, sum2);

        sum.into()
    }
}

fn part_two_2(input: &str) -> i32 {
    let mut i = 0;
    let chars: Vec<_> = input.chars().collect();

    let mut sum = 0;
    let mut enable = true;
    loop {
        match &chars[i..] {
            ['m', 'u', 'l', '(', ..] if enable => {
                i += 4;
                match parse_mul(&mut i, &chars) {
                    Some(d) => sum += d,
                    None => (),
                }
            }
            ['d', 'o', '(', ')', ..] => {
                i += 4;
                enable = true;
            }
            ['d', 'o', 'n', '\'', 't', '(', ')', ..] => {
                i += 7;
                enable = false;
            }
            [_, ..] => i += 1,
            [] => break,
        }
    }
    sum
}

fn parse_mul(j: &mut usize, chars: &Vec<char>) -> Option<i32> {
    let mut i = *j;
    let n1 = parse_int(&mut i, chars)?;
    parse_exact(&mut i, chars, ",")?;
    let n2 = parse_int(&mut i, chars)?;
    parse_exact(&mut i, chars, ")")?;
    *j = i;
    Some(n1 * n2)
}

fn parse_int(j: &mut usize, chars: &Vec<char>) -> Option<i32> {
    let mut n = 0;
    let mut once = false;
    while let Some(d) = char::to_digit(chars[*j], 10) {
        once = true;
        n = n * 10 + d as i32;
        *j += 1;
    }
    match once {
        true => Some(n),
        false => None,
    }
}

fn parse_exact(j: &mut usize, chars: &Vec<char>, expect: &str) -> Option<()> {
    for (i, c) in expect.chars().enumerate() {
        if chars[i + *j] != c {
            return None;
        }
    }
    *j += expect.len();
    Some(())
}

/// Let `muls` be a list of pairs of numbers (a,b), each pair represents one
/// match of "mul\((\d+),(\d+)\)" in the input, numbers a,b are values of
/// capture groups respectively. The list is in the order of the matches.
///
/// For all i,j in `0..muls.len()`, i < j, compute sum of all
/// `(a_i * b_j + a_j..b_i)`, where `..` means integer concatenation.
fn part_three(input: &str) -> i64 {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let muls: Vec<_> = pattern
        .captures_iter(input)
        .map(|cap| {
            let g = cap.extract::<2>().1;
            let a: i64 = g[0].parse().unwrap();
            let b: i64 = g[1].parse().unwrap();
            (a, b)
        })
        .collect();

    let mut sum: i64 = 0;
    for i in 0..muls.len() {
        for j in i + 1..muls.len() {
            let (a_i, b_i) = muls[i];
            let (a_j, b_j) = muls[j];

            sum += a_i * b_j + cat(a_j, b_i);
        }
    }

    sum.into()
}
fn cat(a: i64, b: i64) -> i64 {
    let digits = b.to_string().len() as u32;
    a * 10i64.pow(digits) + b
}
