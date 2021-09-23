use crate::{Area, Point, Square};

// Square with width, height and corners
#[derive(Clone)]
pub struct Circle {
    pub x: i64,
    pub y: i64,
    pub radius: i64,
}

impl Circle {
    pub fn new(x: i64, y: i64, radius: i64) -> Self {
        Circle { x, y, radius }
    }
}

impl Area for Circle {
    fn intersects<A>(&self, other: &A) -> bool
    where
        A: Area,
    {
        let other = other.get_points();
        for point in self.get_points() {
            if other.contains(&point) {
                return true;
            }
        }
        false
    }
    fn intersects_square(&self, other: &Square) -> bool {
        self.intersects(other)
    }
    fn get_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        let x = self.x;
        let y = self.y;
        let r = self.radius;

        for i in (x - r)..=(x + r) {
            for j in (y - r)..=(y + r) {
                let d = (((i - x) * (i - x) + (j - y) * (j - y)) as f32).sqrt();
                // if (i - x) * (i - x) + (j - y) * (j - y) <= r * r {
                if d <= r as f32 {
                    points.push(Point { x: i, y: j });
                }
            }
        }

        points
    }
    fn get_random_point(&self) -> Point {
        use rand::seq::SliceRandom;
        self.get_points()
            .choose(&mut rand::thread_rng())
            .map_or_else(Point::default, |p| *p)
    }
    fn x1(&self) -> i64 {
        self.x - self.radius
    }
    fn y1(&self) -> i64 {
        self.y - self.radius
    }
}
