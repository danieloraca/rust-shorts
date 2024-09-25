use std::fmt;
use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Diff: P.x = {}; P.y = {}", self.x, self.y)
    }
}

fn main() {
    let p1: Point = Point { x: 1, y: 2 };
    let p2: Point = Point { x: 5, y: 10 };

    let p3: Point = p1 + p2;

    println!("{}", p3);
}

