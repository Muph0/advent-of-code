use crate::{Answer, Solution};

#[derive(Clone, Copy)]
pub struct Day02;

fn read_reports(input: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let report = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        reports.push(report);
    }

    reports
}

fn is_safe(report: &Vec<i32>) -> bool {
    report.windows(2).all(|span| {
        let diff = span[0] - span[1];
        diff >= 1 && diff <= 3
    }) || report.windows(2).all(|span| {
        let diff = span[1] - span[0];
        diff >= 1 && diff <= 3
    })
}

impl Solution for Day02 {
    fn day_number(&self) -> i32 {
        2
    }
    fn clone_dyn(&self) -> Box<dyn Solution> {
        Box::new(*self)
    }

    fn part_one(&self, input: &str) -> Answer {
        let reports = read_reports(input);
        let count = reports.into_iter().filter(is_safe).count();
        count.into()
    }

    fn part_two(&self, input: &str) -> Answer {
        let reports = read_reports(input);
        let count = reports
            .into_iter()
            .filter(|rep| {
                for i in 0..rep.len() {
                    let mut omitted = rep.clone();
                    omitted.remove(i);
                    if is_safe(&omitted) {
                        return true;
                    }
                }
                false
            })
            .count();
        count.into()
    }
}
