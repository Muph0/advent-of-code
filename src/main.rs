mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod utils;
mod vec;
mod day07;

use std::{
    fmt::{Display, Write},
    fs, thread,
};

pub enum Answer {
    Int(i64),
}
impl From<i64> for Answer {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
impl From<i32> for Answer {
    fn from(value: i32) -> Self {
        Self::Int(value as _)
    }
}
impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        Self::Int(value as _)
    }
}
impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::Int(val) => val.fmt(f),
        }
    }
}

pub trait Solution: Send {
    fn day_number(&self) -> i32;
    fn clone_dyn(&self) -> Box<dyn Solution>;
    fn part_one(&self, input: &str) -> Answer;
    fn part_two(&self, input: &str) -> Answer;
}

const RST: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GRN: &str = "\x1b[32m";
const YLW: &str = "\x1b[2;33m";

fn main() {
    let solutions: Vec<Box<dyn Solution>> = vec![
        //Box::new(day01::Day01),
        //Box::new(day02::Day02),
        //Box::new(day03::Day03),
        //Box::new(day04::Day04),
        //Box::new(day05::Day05),
        //Box::new(day06::Day06),
        Box::new(day07::Day07),
    ];

    for sol in solutions {
        let day = sol.day_number();

        let mut msg = String::new();
        let input_file = format!("src/day{day:02}.txt");
        msg.write_fmt(format_args!("Task {YLW}{input_file}{RST}"))
            .unwrap();

        let input = match fs::read_to_string(&input_file) {
            Ok(s) => {
                println!("{}, starting...", &msg);
                s
            }
            Err(e) => {
                println!("{}, no input: {}", &msg, e);
                continue;
            }
        };
        let input_ref: &'static str = input.leak();

        let part1_name = format!("{YLW}{}{RST}, part one", &input_file);
        let part1_sol = sol.clone_dyn();
        run_guarded(part1_name.clone(), move || {
            let result = part1_sol.part_one(input_ref);
            println!("     {part1_name} = {GRN}{}{RST}", &result);
        });

        let part2_name = format!("{YLW}{}{RST}, part two", &input_file);
        run_guarded(part2_name.clone(), move || {
            let result = sol.part_two(input_ref);
            println!("     {part2_name} = {GRN}{}{RST}", &result);
        });
    }
}

fn run_guarded<F>(name: String, f: F)
where
    F: Fn() + Send + 'static,
{
    let builder = thread::Builder::new().name(name.clone());
    let thread = builder.spawn(move || f()).unwrap();

    let result = thread.join();
    match result {
        Ok(_) => (),
        Err(_) => (), //println!("     {} {RED}panicked{RST}.", &name),
    }
}
