use bevy::{
    color::palettes::basic::WHITE, prelude::*, sprite::MaterialMesh2dBundle
};
use avian2d::{math::Vector, prelude::*};

pub struct BallPlugin;

#[derive(Component)]
pub struct Ball;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_ball);
        app.add_systems(Update, (movement, accelerate));
    }
}

pub fn add_ball(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
   commands.spawn((
        LinearVelocity(Vector{ x: 1.0, y: -1.0 }),
        RigidBody::Dynamic,
        Collider::circle(10.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(10.0)).into(),
            material: materials.add(Color::from(WHITE)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Ball,
    ));
}

pub fn movement(mut query: Query<(&LinearVelocity, &mut Transform, &Ball)>) {
    for (linear_velocity, mut transform, _ball) in &mut query {
        transform.translation = Vec3::new(transform.translation.x + linear_velocity.x, transform.translation.y + linear_velocity.y, transform.translation.z);
    }
}

pub fn accelerate(mut query: Query<&mut LinearVelocity, With<Ball>>) {
    for mut linear_velocity in &mut query {
        println!("linear_velocity: {}, {}", linear_velocity.x, linear_velocity.y);


        // Calculate the magnitude of the velocity vector
        let mag: f32 = ((linear_velocity.x * linear_velocity.x) + (linear_velocity.y * linear_velocity.y)).sqrt();

        // Calculate the angle from the velocity vector magnitude
        let angle: f32 = f32::acos(linear_velocity.x / mag);

        // New speed increments
        let speed_inc: f32 = 0.1;
        let incx: f32 = (speed_inc * f32::cos(angle)) * (linear_velocity.x / linear_velocity.x.abs());
        let incy: f32 = (speed_inc  * f32::sin(angle)) * (linear_velocity.y / linear_velocity.y.abs());

        println!("new speed increments: {}, {}", incx, incy);

        // Add new speed increments
        linear_velocity.x += incx;
        linear_velocity.y += incy;
    }
}
