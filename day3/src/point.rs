#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32
}

impl Point {

    pub fn new(x: u32, y: u32) -> Point {
        Point { x: x, y: y }
    }
}