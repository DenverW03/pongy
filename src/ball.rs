use bevy::{
    color::palettes::basic::WHITE, prelude::*, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}
};
use avian2d::{math::Vector, prelude::*};

pub struct BallPlugin;

//#[derive(Bundle)]
//pub struct BallBundle<M> where M: Material2d {
//    sprite: MaterialMesh2dBundle<M>,
//    velocity: BallVelocity,
//}

#[derive(Component)]
pub struct Ball;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_ball);
        app.add_systems(Update, movement);
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
        println!("{}", linear_velocity.x);
        println!("{}", linear_velocity.y);


        transform.translation = Vec3::new(transform.translation.x + linear_velocity.x, transform.translation.y + linear_velocity.y, transform.translation.z);
    }
}
