use crate::{Area, Circle, Point, Square};

#[derive(Clone)]
pub struct RoomCircle {
    pub area: Circle,
    pub center: Point,
}

impl RoomCircle {
    pub fn new(area: Circle) -> Self {
        RoomCircle {
            area: area.clone(),
            center: Point::new(area.x, area.y),
        }
    }
}

impl Area for RoomCircle {
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
