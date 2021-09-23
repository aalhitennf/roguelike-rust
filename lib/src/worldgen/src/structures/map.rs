use gdnative::core_types::Vector2;

use crate::{Area, Point, Tile};

#[derive(Debug, Clone, Copy)]
pub enum MapType {
    SquareRooms,
    RoundRooms,
}

pub trait MapInterface {
    fn get_type(&self) -> &MapType;
    fn get_width(&self) -> i64;
    fn get_height(&self) -> i64;
    fn get_matrix(&self) -> &Vec<Tile>;
    fn get_matrix_mut(&mut self) -> &mut Vec<Tile>;
    fn get_tile_coords(&self, p: &Point) -> Option<&Tile>;
    fn get_tile_coords_mut(&mut self, p: &Point) -> Option<&mut Tile>;
    fn get_player_spawnpoint(&self) -> Vector2;
    fn set_matrix_value(&mut self, p: &Point, v: i64);
}

#[derive(Clone)]
pub struct Map {
    pub map_type: MapType,
    pub width: i64,
    pub height: i64,
    pub matrix: Vec<Tile>,
}

// Map is generic "container" for every map type
impl Map {
    pub fn new(width: i64, height: i64, map_type: MapType) -> Self {
        Map {
            map_type,
            width,
            height,
            matrix: create_matrix(width, height),
        }
    }
    #[allow(dead_code)]
    pub fn set_type_area<A: Area>(&mut self, area: &A, t: i64) {
        for point in &area.get_points() {
            if let Some(tile) = self
                .matrix
                .get_mut((point.x + point.y * self.width) as usize)
            {
                tile.tile_type = t;
            }
        }
    }
    #[allow(dead_code)]
    pub fn set_type_points(&mut self, points: Vec<Point>, t: i64) {
        for point in points {
            if let Some(tile) = self
                .matrix
                .get_mut((point.x + point.y * self.width) as usize)
            {
                tile.tile_type = t;
            }
        }
    }
    pub fn set_floor_area<A: Area>(&mut self, area: &A) {
        for point in &area.get_points() {
            if let Some(tile) = self
                .matrix
                .get_mut((point.x + point.y * self.width) as usize)
            {
                tile.set_floor();
            }
        }
    }
    pub fn set_floor_points(&mut self, points: Vec<Point>) {
        for point in points {
            if let Some(tile) = self
                .matrix
                .get_mut((point.x + point.y * self.width) as usize)
            {
                tile.set_floor();
            }
        }
    }
}

fn create_matrix(width: i64, height: i64) -> Vec<Tile> {
    let mut matrix: Vec<Tile> = Vec::new();
    for x in 0..width {
        for y in 0..height {
            matrix.push(Tile::new(x, y));
        }
    }
    matrix
}
