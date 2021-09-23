mod console;

use console::Console;

use gdnative::{godot_init, nativescript::InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Console>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
