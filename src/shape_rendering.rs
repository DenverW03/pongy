use bevy::{
    color::palettes::basic::WHITE, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, utils::warn
};

pub struct ShapePlugin;

#[derive(Component)]
pub struct Shape;

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct Velocity {
    xvel: f32,
    yvel: f32,
}

#[derive(Component)]
pub struct Sprite(Mesh2dHandle);

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_rectangle);
        app.add_systems(Update, apply_velocity);
    }
}

pub fn add_rectangle(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default()); // adding a camera for the meshes
    //commands.spawn((Shape, Position{ x: 100.0, y: 100.0 }, Sprite(Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0))))));
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0))),
        transform: Transform::default().with_scale(Vec3 { x: 100.0, y: 100.0, z: 0.0 }),
        material: materials.add(Color::from(WHITE)),
        ..default()
    });
}

// Querying all entities
pub fn apply_velocity(mut query: Query<(&mut Transform, &Mesh2dHandle)>, keys: Res<ButtonInput<KeyCode>>) {
    for (mut transform, _mesh) in &mut query {
        if keys.just_pressed(KeyCode::ArrowUp) {
            transform.translation = Vec3::new(transform.translation.x, transform.translation.y + 10.0, transform.translation.z);
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            transform.translation = Vec3::new(transform.translation.x, transform.translation.y - 10.0, transform.translation.z);
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            transform.translation = Vec3::new(transform.translation.x + 10.0, transform.translation.y, transform.translation.z);
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            transform.translation = Vec3::new(transform.translation.x - 10.0, transform.translation.y, transform.translation.z);
        }
    }
}
