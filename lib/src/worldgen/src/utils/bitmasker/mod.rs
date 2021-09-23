mod masker;

use masker::DungeonTilesetBitmaskValues;

use crate::{MapInterface, MapType, Point, Result};

const DUNGEON_BITMASK_VALUES: &str = "assets/tileset/dungeon/bitmask_values.json";

#[derive(Clone, Debug)]
pub enum BitmaskType {
    Dungeon(Box<DungeonTilesetBitmaskValues>),
}

pub fn apply<T>(map: &mut Box<T>) -> Result<()>
where
    T: MapInterface + ?Sized,
{
    let mask_data = read_bitmask_data(map.get_type())?;
    let coords = get_unused_coords(map.as_ref());
    set_matrix_values(&coords, mask_data, map);
    Ok(())
}

fn read_bitmask_data(_t: &MapType) -> Result<BitmaskType> {
    // let path = match *t as u32 {
    //     0 | 1 => DUNGEON_VALUES,
    //     _ => DUNGEON_VALUES,
    // };

    let path = DUNGEON_BITMASK_VALUES;

    let content = std::fs::read_to_string(path)?;
    let data: DungeonTilesetBitmaskValues = serde_json::from_str(&content)?;

    Ok(BitmaskType::Dungeon(Box::new(data)))
}

#[allow(clippy::borrowed_box)]
fn coord_out_of_bounds(point: &Point, m_width: i64, m_height: i64) -> bool {
    point.x < 0 || point.y < 0 || point.x > m_width || point.y > m_height
}

#[allow(clippy::borrowed_box)]
fn coord_is_floor<T>(x: i64, y: i64, map: &Box<T>) -> bool
where
    T: MapInterface + ?Sized,
{
    let p = Point::new(x, y);

    if coord_out_of_bounds(&p, map.get_width(), map.get_height()) {
        return false;
    }

    if let Some(tile) = map.get_tile_coords(&p) {
        return tile.tile_type == 2;
    }

    false
}

#[allow(clippy::borrowed_box)]
pub fn get_bitmask_8bit<T>(p: &Point, map: &Box<T>) -> i64
where
    T: MapInterface + ?Sized,
{
    let mut f = 0;
    if coord_is_floor(p.x - 1, p.y - 1, map) {
        f += 1;
    };
    if coord_is_floor(p.x, p.y - 1, map) {
        f += 2;
    };
    if coord_is_floor(p.x + 1, p.y - 1, map) {
        f += 4;
    };
    if coord_is_floor(p.x - 1, p.y, map) {
        f += 8;
    };
    if coord_is_floor(p.x + 1, p.y, map) {
        f += 16;
    };
    if coord_is_floor(p.x - 1, p.y + 1, map) {
        f += 32;
    };
    if coord_is_floor(p.x, p.y + 1, map) {
        f += 64;
    };
    if coord_is_floor(p.x + 1, p.y + 1, map) {
        f += 128;
    };
    f
}

// Gets all coords that aren't floor
fn get_unused_coords<T>(map: &T) -> Vec<Point>
where
    T: MapInterface + ?Sized,
{
    let w = map.get_width() as usize;
    let h: usize = map.get_height() as usize;
    let matrix = map.get_matrix();
    let mut coords = Vec::new();
    let mut i: usize;
    for x in 0..=w {
        for y in 0..=h {
            i = x + y * w;
            if let Some(tile) = matrix.get(i) {
                if tile.tile_type != 2 {
                    coords.push(Point::new(x as i64, y as i64));
                }
            }
        }
    }
    coords
}

// Set actual values for the tilemap
fn set_matrix_values<T>(coords: &[Point], mask_data: BitmaskType, map: &mut Box<T>)
where
    T: MapInterface + ?Sized,
{
    match mask_data {
        BitmaskType::Dungeon(data) => masker::dungeon(coords, &data, map),
        #[allow(unreachable_patterns)]
        _ => panic!("Bitmas data not initialized for type {:?}", mask_data),
    }
}
