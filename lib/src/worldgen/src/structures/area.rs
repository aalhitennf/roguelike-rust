use crate::{Point, Square};

// Area is trait for different types of areas ;D
pub trait Area {
    fn intersects<A: Area>(&self, r: &A) -> bool;
    // ? This could probably be done in the previous fn too
    fn intersects_square(&self, s: &Square) -> bool;
    fn get_points(&self) -> Vec<Point>;
    fn get_random_point(&self) -> Point;
    fn x1(&self) -> i64;
    fn y1(&self) -> i64;
}
