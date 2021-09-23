mod error;
mod map_types;
mod structures;
mod tilemap;
mod utils;

use gdnative::{godot_init, nativescript::InitHandle};

use structures::{Area, Circle, Map, MapInterface, MapType, Point, Square, Tile};
use tilemap::StaticTileMap;

pub type Result<T> = std::result::Result<T, error::Error>;
// pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<StaticTileMap>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
