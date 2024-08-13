use bevy::{
    color::palettes::basic::WHITE, prelude::*, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}
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
        LinearVelocity(Vector{ x: 1.0, y: 0.0 }),
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

// TODO: MAKE THE ACCELERATION ACCOUNT FOR THE CURRENT VECTOR, ACCELERATE LINEARLY
pub fn accelerate(mut query: Query<&mut LinearVelocity, With<Ball>>) {
    for mut linear_velocity in &mut query {
        // Inline condiitonals to account for direction
        let incx = if linear_velocity.x > 0.0 {0.1} else {-0.1};
        let incy = if linear_velocity.y > 0.0 {0.1} else {-0.1};

        linear_velocity.x += incx;
        linear_velocity.y += incy;
    }
}

//pub fn handle_collision(collisions: Res<Collisions>, mut query: Query<&mut LinearVelocity, With<Ball>>) {
//    // This loop will only occur once as there is only one ball
//    for mut linear_velocity in &mut query {
//        for _contacts in collisions.iter() {
//            linear_velocity.x *= 2.0;
//            linear_velocity.y *= 2.0;
//        }
//    }
//}
