use point::Point;

#[derive(PartialEq, Debug)]
pub struct Rectangle {
    pub start: Point,
    pub width: u32,
    pub height: u32
}

impl Rectangle {

    pub fn new(start: Point, width: u32, height: u32) -> Rectangle {
        Rectangle { start: start, width: width, height: height }
    }
}