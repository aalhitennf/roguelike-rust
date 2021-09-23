use gdnative::{api::TileMap, prelude::*};

use crate::{map_types, utils::bitmasker, MapInterface, Point};

#[derive(NativeClass)]
#[inherit(TileMap)]
pub struct StaticTileMap {
    map: Option<Box<dyn MapInterface>>,
}

impl StaticTileMap {
    fn draw_tiles(&self, owner: &TileMap) {
        self.map.as_ref().map_or_else(
            || {
                godot_print!("[libworldgen::StaticTileMap] draw_tiles failed: no map.");
            },
            |map| {
                owner.clear();

                for x in 0..=map.get_width() {
                    for y in 0..=map.get_height() {
                        // Not most efficient way to create point here
                        if let Some(tile) = map.get_tile_coords(&Point::new(x, y)) {
                            owner.set_cell(
                                x,
                                y,
                                tile.tile_type,
                                false,
                                false,
                                false,
                                Vector2::default(),
                            );
                        }
                    }
                }
            },
        );
    }
}

#[methods]
impl StaticTileMap {
    fn new(_owner: &TileMap) -> Self {
        StaticTileMap { map: None }
    }
    // TODO needs refactor
    #[export]
    fn initialize(&mut self, owner: &TileMap, map_type: i64, w: i64, h: i64) -> String {
        godot_print!("we here");
        let message: String;
        match map_type {
            0 => {
                self.map = Some(Box::new(map_types::MapSquareRooms::new(w, h)));
                message = "Map created succesfully".into();
            }
            1 => {
                self.map = Some(Box::new(map_types::MapCircleRooms::new(w, h)));
                message = "Map created succesfully".into();
            }
            2 => {
                self.map = Some(Box::new(map_types::MapMixedRooms::new(w, h)));
                message = "Map created succesfully".into();
            }
            _ => message = format!("Invalid map type: {}", map_type),
        }

        self.mask(owner);

        message
    }
    #[export]
    fn mask(&mut self, owner: &TileMap) {
        if let Some(map) = self.map.as_mut() {
            if let Err(e) = bitmasker::apply(map) {
                godot_print!(
                    "[libworldgen::StaticTileMap] Failed to apply bitmasking: {}",
                    e.to_string()
                );
            } else {
                self.draw_tiles(owner);
            }
        }
    }
    #[export]
    fn clear(&self, owner: &TileMap) {
        owner.clear();
    }
    #[export]
    fn get_player_spawnpoint(&self, _: &TileMap) -> Vector2 {
        if let Some(map) = &self.map {
            return map.get_player_spawnpoint();
        }
        panic!("Failed to get player spawnpoint");
    }
    // #[export]
    fn get_point_bitmask_value(&self, point: Point) -> i64 {
        self.map
            .as_ref()
            .map_or_else(|| 0, |map| bitmasker::get_bitmask_8bit(&point, map))
    }
    #[export]
    fn _process(&self, owner: &TileMap, _delta: f32) {
        let input = Input::godot_singleton();

        if Input::is_action_just_pressed(input, "mouse_scroll_down") {
            self.change_tile(owner, -1);
        }

        if Input::is_action_just_pressed(input, "mouse_right") {
            self.change_tile(owner, 1);
        }
    }
    #[allow(clippy::unused_self)]
    fn get_mouse_coords(&self, owner: &TileMap) -> Vector2 {
        owner.world_to_map(owner.get_global_mouse_position())
    }
    fn change_tile(&self, owner: &TileMap, to: i64) {
        let tileset = owner.tileset().unwrap();

        let coords = self.get_mouse_coords(owner);
        godot_print!("Coords: {:?}", coords);

        let bitmask_value = self.get_point_bitmask_value(Point::from(coords));
        godot_print!("Bitmask value: {:?}", bitmask_value);

        let current_value = owner.get_cellv(coords);

        let max = unsafe { tileset.assume_safe().get_tiles_ids().len() as i64 };

        let mut new_value = current_value + to;

        if new_value > max - 1 {
            new_value = 0;
        }

        if new_value < 0 {
            new_value = max - 1;
        }

        let new_name = unsafe { tileset.assume_safe().tile_get_name(new_value) };

        godot_print!("New value: {}", new_value);
        godot_print!("New name: {}", new_name);

        owner.set_cellv(coords, new_value, false, false, false);
    }
}
