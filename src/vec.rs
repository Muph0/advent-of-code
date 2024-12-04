// Hello there, fellow coders! 👋
//
// This file is brought to you by your friendly AI assistant, ChatGPT.
// I don’t sleep, I don’t eat, but I sure do love writing code for you!
// Feel free to use, modify, and share this code—it’s here to help you and your projects thrive.
//
// Just remember: while I’m pretty smart, I’m not perfect. Always review, test, and make it your own.
// Happy coding, and may your bugs be few and your builds swift!
//
// Cheers, ChatGPT
#![allow(unused)]
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    // Constructor
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Distance: Euclidean
    pub fn dist(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    // Distance: Squared Euclidean
    pub fn dist_sq(&self, other: &Self) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }

    // Distance: Manhattan
    pub fn dist_manhattan(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    // Dot Product
    pub fn dot(&self, other: &Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    // Rotate 90 degrees to the left (counterclockwise)
    pub fn left90(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    // Rotate 90 degrees to the right (clockwise)
    pub fn right90(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn directions_8() -> impl Iterator<Item = Vec2i> {
        [
            Vec2i::new(0, 1),   // Up
            Vec2i::new(1, 1),   // Up-right
            Vec2i::new(1, 0),   // Right
            Vec2i::new(1, -1),  // Down-right
            Vec2i::new(0, -1),  // Down
            Vec2i::new(-1, -1), // Down-left
            Vec2i::new(-1, 0),  // Left
            Vec2i::new(-1, 1),  // Up-left
        ]
        .into_iter()
    }

    pub fn directions_4() -> impl Iterator<Item = Vec2i> {
        [
            Vec2i::new(0, 1),  // Up
            Vec2i::new(1, 0),  // Right
            Vec2i::new(0, -1), // Down
            Vec2i::new(-1, 0), // Left
        ]
        .into_iter()
    }

    pub(crate) fn directions_4_diag() -> impl Iterator<Item = Vec2i> {
        [
            Vec2i::new(1, 1),   // Up-right
            Vec2i::new(1, -1),  // Down-right
            Vec2i::new(-1, -1), // Down-left
            Vec2i::new(-1, 1),  // Up-left
        ]
        .into_iter()
    }
}

// Vector-vector addition
impl Add for Vec2i {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Vector-vector subtraction
impl Sub for Vec2i {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<I: Into<i32>> Mul<I> for Vec2i {
    type Output = Self;

    fn mul(self, scalar: I) -> Self {
        let i: i32 = scalar.into();
        Self {
            x: self.x * i,
            y: self.y * i,
        }
    }
}

impl Div<i32> for Vec2i {
    type Output = Self;

    fn div(self, scalar: i32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

// Into<(i32, i32)>
impl Into<(i32, i32)> for Vec2i {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

// From<(i32, i32)>
impl From<(i32, i32)> for Vec2i {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
