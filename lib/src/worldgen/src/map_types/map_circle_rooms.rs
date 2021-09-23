use gdnative::prelude::*;
use rand::Rng;

use crate::Circle;
use crate::{structures::rooms::RoomCircle, Area, Map, MapInterface, MapType, Point, Tile};

#[derive(Clone)]
pub struct MapCircleRooms {
    pub map: Map,
    min_room_width: i64,
    max_room_width: i64,
    min_room_height: i64,
    max_room_height: i64,
    list_of_rooms: Vec<RoomCircle>,
}

impl MapInterface for MapCircleRooms {
    // Gets
    fn get_type(&self) -> &MapType {
        &self.map.map_type
    }
    fn get_width(&self) -> i64 {
        self.map.width
    }
    fn get_height(&self) -> i64 {
        self.map.height
    }
    fn get_matrix(&self) -> &Vec<Tile> {
        &self.map.matrix
    }
    fn get_matrix_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.map.matrix
    }
    fn get_tile_coords(&self, p: &Point) -> Option<&Tile> {
        self.map.matrix.get((p.x + p.y * self.map.width) as usize)
    }
    fn get_tile_coords_mut(&mut self, p: &Point) -> Option<&mut Tile> {
        self.map
            .matrix
            .get_mut((p.x + p.y * self.map.width) as usize)
    }
    fn get_player_spawnpoint(&self) -> Vector2 {
        self.list_of_rooms
            .first()
            .map_or_else(Vector2::default, |room| {
                let p = room.get_random_point();
                Vector2::new(p.x as f32, p.y as f32)
            })
    }
    // Sets
    fn set_matrix_value(&mut self, p: &Point, v: i64) {
        match self
            .map
            .matrix
            .get_mut((p.x + p.y * self.map.width) as usize)
        {
            None => godot_print!(
                "[libworldgen::MapInteface] set_matrix_value: invalid point {:?}",
                p
            ),
            Some(tile) => tile.tile_type = v,
        }
    }
}

impl MapCircleRooms {
    pub fn new(w: i64, h: i64) -> Self {
        let mut new_map = MapCircleRooms {
            map: Map::new(w, h, MapType::RoundRooms),
            min_room_width: 0,
            max_room_width: 0,
            min_room_height: 0,
            max_room_height: 0,
            list_of_rooms: Vec::new(),
        };
        new_map.initialize();
        new_map
    }
    pub fn initialize(&mut self) {
        // godot_print!("initializing");
        self.set_room_limits();
        // godot_print!("limits set");
        self.create_rooms_no_intersect();
        // godot_print!("rooms created");
        // godot_print!("{} rooms", self.list_of_rooms.len());
        self.create_tunnels();
        // godot_print!("tunnels created");
        godot_print!(
            "[libworldgen::MapCircleRooms] New map ({}x{}) initialized!",
            self.map.width,
            self.map.height
        );
    }
    fn set_room_limits(&mut self) {
        self.min_room_width = 3;
        self.max_room_width = 8;
        self.min_room_height = 3;
        self.max_room_height = 8;
    }
    fn create_rooms_no_intersect(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            // Roll room dimensions and position
            let radius = rng.gen_range(self.min_room_height..self.max_room_height);
            let x = rng.gen_range((radius + 1)..(self.map.width - (radius + 1)));
            let y = rng.gen_range((radius + 1)..(self.map.height - (radius + 1)));
            // Create room
            let possible_location = Circle::new(x, y, radius);
            let mut failed = false;
            // Check if room intersect with existing on
            // and break loop if so
            for room in self.list_of_rooms.iter() {
                if possible_location.intersects(&room.area) {
                    failed = true;
                    break;
                }
            }
            // godot_print!("intersect tested");
            // Try again if failed
            if failed {
                continue;
            };
            // godot_print!("didnt intersect");
            // Location fits the map so create room and
            // add to matrix
            let new_room = RoomCircle::new(possible_location);
            // godot_print!("new room created");
            self.map.set_floor_area(&new_room.area);
            // godot_print!("map_area added");
            // Add room tiles and add to list
            self.list_of_rooms.push(new_room);
            // godot_print!("added to list of room");
        }
    }
    fn create_tunnels(&mut self) {
        use std::cmp::{max, min};

        let mut rng = rand::thread_rng();
        let mut points = Vec::new();

        #[allow(unused_assignments)]
        let (mut sx, mut ex, mut sy, mut ey) = (0, 0, 0, 0);

        for (i, current_room) in self.list_of_rooms.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let previous_room = &self.list_of_rooms.get(i - 1);

            if previous_room.is_none() {
                godot_error!("[libworldgen::create_tunnels] Failed to create tunnels: Failed to get previous center");
                continue;
            }

            let previous_center = &previous_room.unwrap().center;

            sx = min(current_room.area.x, previous_center.x);
            ex = max(current_room.area.x, previous_center.x) + 1;
            sy = min(current_room.area.y, previous_center.y);
            ey = max(current_room.area.y, previous_center.y) + 1;

            if rng.gen::<bool>() {
                for x in sx..ex {
                    points.push(Point::new(x, current_room.area.y));
                }
                for y in sy..ey {
                    points.push(Point::new(previous_center.x, y));
                }
            } else {
                for y in sy..ey {
                    points.push(Point::new(current_room.area.x, y));
                }
                for x in sx..ex {
                    points.push(Point::new(x, previous_center.y));
                }
            }
        }
        self.map.set_floor_points(points);
    }
}
