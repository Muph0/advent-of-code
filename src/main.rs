mod day01;
mod day02;
mod day03;

use std::{fmt::Write, fs, thread};

pub trait Solution: Send {
    fn day_number(&self) -> i32;
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

fn main() {
    let solutions: Vec<Box<dyn Solution>> = vec![
        //
        Box::new(day01::Day01),
        Box::new(day02::Day02),
    ];

    for sol in solutions {
        let day = sol.day_number();

        let mut msg = String::new();
        let input_file = format!("src/day{day:02}.txt");
        msg.write_fmt(format_args!("Task \"{input_file}\""))
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

        let builder = thread::Builder::new().name(input_file.clone());
        let thread = builder
            .spawn(move || {
                fs::write(format!("src/day{day:02}.out1.txt"), sol.part_one(&input)).unwrap();
                fs::write(format!("src/day{day:02}.out2.txt"), sol.part_two(&input)).unwrap();
            })
            .unwrap();

        let result = thread.join();
        match result {
            Ok(_) => println!("{} completed.", &msg),
            Err(e) => println!("{} panicked with {e:?}.", &msg),
        }
    }
}
