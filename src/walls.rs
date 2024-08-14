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

fn add_walls(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, window: Query<&Window>) {
    // Getting the window attributes for wall placement
    if let Ok(window) = window.get_single() {
        let width = window.resolution.width();
        let height = window.resolution.height();

        let wall_thickness = 10.0;
        let left_side = 0.0 - (width / 2.0) + (wall_thickness / 2.0);
        let right_side = 0.0 + (width / 2.0) - (wall_thickness / 2.0);
        let bottom_side = 0.0 - (height / 2.0) + (wall_thickness / 2.0);
        let top_side = 0.0 + (height / 2.0) - (wall_thickness / 2.0);

        // Left
        commands.spawn((
            RigidBody::Static,
            Collider::rectangle(wall_thickness, height),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(wall_thickness, height))),
                transform: Transform::from_xyz(left_side, 0.0, 0.0),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
        ));
        // Right
        commands.spawn((
            RigidBody::Static,
            Collider::rectangle(wall_thickness, height),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(wall_thickness, height))),
                transform: Transform::from_xyz(right_side, 0.0, 0.0),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
        ));
        // Top
        commands.spawn((
            RigidBody::Static,
            Collider::rectangle(width, wall_thickness),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(width, wall_thickness))),
                transform: Transform::from_xyz(0.0, top_side, 0.0),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
        ));
        // Bottom
        commands.spawn((
            RigidBody::Static,
            Collider::rectangle(width, wall_thickness),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(width, wall_thickness))),
                transform: Transform::from_xyz(0.0, bottom_side, 0.0),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
        ));
    }
}
