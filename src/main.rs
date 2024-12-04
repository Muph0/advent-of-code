mod day01;
mod day02;
mod day03;
mod day04;
mod vec;

use std::{fmt::Write, fs, thread};

pub trait Solution: Send {
    fn day_number(&self) -> i32;
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
    fn clone_dyn(&self) -> Box<dyn Solution>;
}

fn main() {
    let solutions: Vec<Box<dyn Solution>> = vec![
        //
        //Box::new(day01::Day01),
        //Box::new(day02::Day02),
        //Box::new(day03::Day03),
        Box::new(day04::Day04),
    ];

    for sol in solutions {
        let day = sol.day_number();

        let mut msg = String::new();
        let input_file = format!("src/day{day:02}.txt");
        msg.write_fmt(format_args!("Task {input_file}"))
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

        let part1_name = format!("{}, part one", &input_file);
        let part1_sol = sol.clone_dyn();
        run_guarded(part1_name.clone(), move || {
            let result = part1_sol.part_one(input_ref);
            println!("     {part1_name} = {}", &result);
        });

        let part2_name = format!("{}, part two", &input_file);
        run_guarded(part2_name.clone(), move || {
            let result = sol.part_two(input_ref);
            println!("     {part2_name} = {}", &result);
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
        Err(_) => println!("{} panicked.", &name),
    }
}
