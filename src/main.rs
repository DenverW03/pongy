use bevy::prelude::*;
use shape_rendering::ShapePlugin;

pub mod shape_rendering;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .run();
}
