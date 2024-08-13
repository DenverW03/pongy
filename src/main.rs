use bevy::prelude::*;
use avian2d::{math::Vector, prelude::*};
use shape_rendering::PaddlePlugin;
use ball::BallPlugin;

pub mod shape_rendering;
pub mod ball;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vector{ x: 0.0, y: 0.0 })) // no gravity
        .add_plugins((PaddlePlugin, BallPlugin))
        .run();
}
