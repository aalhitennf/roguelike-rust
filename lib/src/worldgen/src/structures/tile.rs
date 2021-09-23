use crate::Point;

#[derive(Clone)]
pub struct Tile {
    pub coords: Point,
    pub in_los: bool,
    pub seen: bool,
    pub blocks_path: bool,
    pub blocks_vision: bool,
    pub tile_type: i64,
}

impl Tile {
    pub fn new(x: i64, y: i64) -> Self {
        Tile {
            coords: Point::new(x, y),
            in_los: false,
            seen: false,
            blocks_path: true,
            blocks_vision: true,
            tile_type: -1,
        }
    }
    pub fn set_floor(&mut self) {
        self.blocks_path = false;
        self.blocks_vision = false;
        self.tile_type = 2;
    }
}
