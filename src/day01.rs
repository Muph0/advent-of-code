use crate::Solution;

#[derive(Clone)]
pub struct Day01;

fn read_columns(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    let mut is_left = true;

    for n in input.split_whitespace() {
        let column = match is_left {
            true => &mut left,
            false => &mut right,
        };

        column.push(n.parse().unwrap());
        is_left = !is_left;
    }

    (left, right)
}

impl Solution for Day01 {
    fn day_number(&self) -> i32 {
        1
    }
    fn part_one(&self, input: &str) -> String {
        let (mut left, mut right) = read_columns(input);
        left.sort();
        right.sort();

        let mut sum = 0;
        for (l, r) in left.into_iter().zip(right) {
            sum += (l - r).abs();
        }

        return sum.to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let (left, mut right) = read_columns(input);
        right.sort();

        let mut sum = 0;
        for i in left {
            let upper = right.partition_point(|&n| n <= i) as i32;
            let lower = right.partition_point(|&n| n < i) as i32;

            sum += i * (upper - lower);
        }

        return sum.to_string();
    }
}
