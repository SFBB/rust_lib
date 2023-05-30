use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn main() {
    print!("{:?}\n", Point { x: 3, y: 3 } - Point { x: 2, y: 3 });
}
