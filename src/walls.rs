use bevy::{
    color::palettes::basic::WHITE, prelude::*, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}
};
use avian2d::prelude::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_walls);
    }
}

fn add_walls() {
}
