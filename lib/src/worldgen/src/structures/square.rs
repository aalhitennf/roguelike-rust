use crate::{Area, Point};

// Square with width, height and corners
#[derive(Clone)]
pub struct Square {
    pub width: i64,
    pub height: i64,
    pub x1: i64,
    pub x2: i64,
    pub y1: i64,
    pub y2: i64,
}

impl Square {
    pub fn new(width: i64, height: i64, x1: i64, y1: i64) -> Self {
        Square {
            width,
            height,
            x1,
            x2: x1 + width,
            y1,
            y2: y1 + height,
        }
    }
}

impl Area for Square {
    fn intersects<A: Area>(&self, other: &A) -> bool {
        let other = other.get_points();
        for point in self.get_points() {
            if other.contains(&point) {
                return true;
            }
        }
        false
    }
    fn intersects_square(&self, other: &Square) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
    fn get_points(&self) -> Vec<Point> {
        let mut tiles: Vec<Point> = vec![Point::default(); (self.width * self.height) as usize];

        let mut i = 0;

        for x in self.x1..self.x2 {
            for y in self.y1..self.y2 {
                tiles[i].x = x;
                tiles[i].y = y;
                i += 1;
            }
        }
        tiles
    }
    fn get_random_point(&self) -> Point {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(self.x1 + 1..=self.x2 - 1);
        let y = rng.gen_range(self.y1 + 1..=self.y2 - 1);
        Point::new(x, y)
    }
    fn x1(&self) -> i64 {
        self.x1
    }
    fn y1(&self) -> i64 {
        self.y1
    }
}
