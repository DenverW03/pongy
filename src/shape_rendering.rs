use bevy::{
    color::palettes::basic::WHITE, prelude::*, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}
};

pub struct PaddlePlugin;

#[derive(Bundle)]
struct PaddleBundle<M> where M: Material2d {
    sprite: MaterialMesh2dBundle<M>,
    velocity: Velocity,
}

#[derive(Component)]
pub struct Velocity {
    xvel: f32,
    yvel: f32,
}

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_rectangle);
        app.add_systems(Update, apply_velocity);
    }
}

pub fn add_rectangle(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // adding a camera for the meshes
    commands.spawn(Camera2dBundle::default());

    // Adding the paddle bundle
    commands.spawn(PaddleBundle {
        sprite: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0))),
            transform: Transform::default().with_scale(Vec3 { x: 100.0, y: 100.0, z: 0.0 }),
            material: materials.add(Color::from(WHITE)),
            ..default()
        },
        velocity: Velocity { xvel: 0.0, yvel: 0.0 },
    });
}

// Querying entities that have the attributes Transform and Mesh2dHandle (essentially the paddles)
pub fn apply_velocity(mut query: Query<(&mut Transform, &Mesh2dHandle)>, keys: Res<ButtonInput<KeyCode>>) {
    for (mut transform, _sprite) in &mut query {
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
