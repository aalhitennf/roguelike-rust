use gdnative::core_types::Vector2;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl From<Vector2> for Point {
    fn from(vector: Vector2) -> Self {
        Point {
            x: vector.x as i64,
            y: vector.y as i64,
        }
    }
}
