use bevy::{
    color::palettes::basic::WHITE, prelude::*, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}
};

pub struct PaddlePlugin;

#[derive(Bundle)]
struct PaddleBundle<M> where M: Material2d {
    sprite: MaterialMesh2dBundle<M>,
    velocity: PaddleVelocity,
    direction: Direction,
    side: Side,
}

#[derive(Component)]
pub struct PaddleVelocity(f32);

#[derive(Component)]
pub struct Direction(f32); // 1.0 or -1.0 to indicate direction of velocity

#[derive(Component)]
pub struct Side(String);

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_rectangle);
        app.add_systems(Update, handle_keys);
    }
}

pub fn add_rectangle(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, window: Query<&Window>) {
    // adding a camera for the meshes
    commands.spawn(Camera2dBundle::default());

    if let Ok(window) = window.get_single() {
        let width = window.resolution.width();
        let padding = 10.0;
        let left_off = (-width / 2.0) + padding;
        let right_off = (width / 2.0) - padding;

        // Adding the left paddle
        commands.spawn(PaddleBundle {
            sprite: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0))),
                transform: Transform::from_xyz(left_off, 0.0, 0.0).with_scale(Vec3 { x: 100.0, y: 100.0, z: 0.0 }),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
            velocity: PaddleVelocity(0.0),
            direction: Direction(0.0),
            side: Side("left".to_string()),
        });

        // Adding the right paddle
        commands.spawn(PaddleBundle {
            sprite: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0))),
                transform: Transform::from_xyz(right_off, 0.0, 0.0).with_scale(Vec3 { x: 100.0, y: 100.0, z: 0.0 }),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
            velocity: PaddleVelocity(0.0),
            direction: Direction(0.0),
            side: Side("right".to_string()),
        });
    }
}

// Querying entities that have the specific attributes (essentially the paddles)
pub fn handle_keys(mut query: Query<(&mut Transform, &Mesh2dHandle, &mut PaddleVelocity, &mut Direction, &Side)>, keys: Res<ButtonInput<KeyCode>>) {
    for (mut transform, _sprite, mut _velocity, mut direction, _side) in &mut query {
        // Setting the direction of velocity based on the key being down
        if keys.pressed(KeyCode::ArrowUp) {
            *direction = Direction(1.0);
        }
        else if keys.pressed(KeyCode::ArrowDown) {
            *direction = Direction(-1.0);
        }
        else {
            *direction = Direction(0.0);
        }

        // Applying the velocity TODO: MAKE THE VELOCITY ACCELERATE AND DECELERATE
        transform.translation = Vec3::new(transform.translation.x, transform.translation.y + (1.0 * direction.0), transform.translation.z);
    }
}
