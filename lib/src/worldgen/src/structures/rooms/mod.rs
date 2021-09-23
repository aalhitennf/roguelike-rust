mod circle_room;
mod square_room;

pub use circle_room::RoomCircle;
pub use square_room::RoomSquare;

use crate::{Area, Point};


#[derive(Clone)]
pub enum Room {
    Circle(RoomCircle),
    Square(RoomSquare),
}

// pub struct Room(RoomType);

impl Area for Room {
    fn intersects<A: Area>(&self, r: &A) -> bool {
        match self {
            Room::Circle(room) => room.intersects(r),
            Room::Square(room) => room.intersects(r),
        }
    }
    fn intersects_square(&self, s: &super::Square) -> bool {
        match self {
            Room::Circle(room) => room.intersects_square(s),
            Room::Square(room) => room.intersects_square(s),
        }
    }
    fn get_points(&self) -> Vec<super::Point> {
        match self {
            Room::Circle(room) => room.get_points(),
            Room::Square(room) => room.get_points(),
        }
    }
    fn get_random_point(&self) -> super::Point {
        match self {
            Room::Circle(room) => room.get_random_point(),
            Room::Square(room) => room.get_random_point(),
        } 
    }
    fn x1(&self) -> i64 {
        match self {
            Room::Circle(room) => room.x1(),
            Room::Square(room) => room.x1(),
        } 
    }
    fn y1(&self) -> i64 {
        match self {
            Room::Circle(room) => room.y1(),
            Room::Square(room) => room.y1(),
        } 
    }
}

impl Room {
    pub fn center(&self) -> Point {
        match self {
            // room => room.,
            Room::Circle(room) => room.center,
            Room::Square(room) => room.center,
        } 
    }
}

// #[derive(Clone)]

// pub struct Room(RoomType);

// impl Area for Room {
//     fn intersects<A: Area>(&self, r: &A) -> bool {
        
//     }
// }