use crate::{Area, Point, Square};

#[derive(Clone)]
pub struct RoomSquare {
    pub area: Square,
    pub center: Point,
}

impl RoomSquare {
    pub fn new(area: Square) -> Self {
        RoomSquare {
            area: area.clone(),
            center: Point::new((area.x1 + area.x2) / 2, (area.y1 + area.y2) / 2),
        }
    }
}

impl Area for RoomSquare {
    fn intersects<A: Area>(&self, other: &A) -> bool {
        self.area.intersects(other)
    }
    fn intersects_square(&self, s: &Square) -> bool {
        self.area.intersects_square(s)
    }
    fn get_points(&self) -> Vec<Point> {
        self.area.get_points()
    }
    fn get_random_point(&self) -> Point {
        self.area.get_random_point()
    }
    fn x1(&self) -> i64 {
        self.area.x1()
    }
    fn y1(&self) -> i64 {
        self.area.y1()
    }
}
