use bevy::prelude::*;
use shape_rendering::PaddlePlugin;

pub mod shape_rendering;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PaddlePlugin)
        .run();
}
