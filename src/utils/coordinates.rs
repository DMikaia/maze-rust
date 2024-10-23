#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Coordinates {
        Self { x, y }
    }
}
