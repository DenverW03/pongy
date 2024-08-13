use bevy::{
    color::palettes::basic::WHITE, prelude::*, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}
};

pub struct PaddlePlugin;

#[derive(Bundle)]
struct PaddleBundle<M> where M: Material2d {
    sprite: MaterialMesh2dBundle<M>,
    speed: PaddleSpeed,
    direction: Direction,
    side: Side,
}

#[derive(Component)]
pub struct PaddleSpeed(f32);

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
        // Calculating where to place the paddles
        let width = window.resolution.width();
        let padding = 100.0;
        let left_off = (-width / 2.0) + padding;
        let right_off = (width / 2.0) - padding;

        // Adding the left paddle
        commands.spawn(PaddleBundle {
            sprite: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0))),
                transform: Transform::from_xyz(left_off, 0.0, 0.0).with_scale(Vec3 { x: 20.0, y: 100.0, z: 0.0 }),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
            speed: PaddleSpeed(5.0), // Base speed is 10.0
            direction: Direction(0.0),
            side: Side("left".to_string()),
        });

        // Adding the right paddle
        commands.spawn(PaddleBundle {
            sprite: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0))),
                transform: Transform::from_xyz(right_off, 0.0, 0.0).with_scale(Vec3 { x: 20.0, y: 100.0, z: 0.0 }),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
            speed: PaddleSpeed(5.0),
            direction: Direction(0.0),
            side: Side("right".to_string()),
        });
    }
}

// Querying entities that have the specific attributes (essentially the paddles)
pub fn handle_keys(mut query: Query<(&mut Transform, &Mesh2dHandle, &mut PaddleSpeed, &mut Direction, &Side)>, keys: Res<ButtonInput<KeyCode>>) {
    for (mut transform, _sprite, mut speed, mut direction, side) in &mut query {
        // If none of the paddle movement keys are pressed then just return
        if !(keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::ArrowDown)) {
            // reset speed to base (momentum lost without key press) and return
            speed.0 = 5.0;
            continue;
        }

        // Setting the direction of velocity based on the key being down
        if side.0 == "left".to_string() {
                if keys.pressed(KeyCode::KeyW) {
                    *direction = Direction(1.0);
                    transform.translation = Vec3::new(transform.translation.x, transform.translation.y + (speed.0 * direction.0), transform.translation.z);
                    speed.0 += 0.06;
                }
                else if keys.pressed(KeyCode::KeyS) {
                    *direction = Direction(-1.0);
                    transform.translation = Vec3::new(transform.translation.x, transform.translation.y + (speed.0 * direction.0), transform.translation.z);
                    speed.0 += 0.06;
                }
        }
        else if side.0 == "right".to_string() {
                if keys.pressed(KeyCode::ArrowUp) {
                    *direction = Direction(1.0);
                    transform.translation = Vec3::new(transform.translation.x, transform.translation.y + (speed.0 * direction.0), transform.translation.z);
                    speed.0 += 0.06;
                }
                else if keys.pressed(KeyCode::ArrowDown) {
                    *direction = Direction(-1.0);
                    transform.translation = Vec3::new(transform.translation.x, transform.translation.y + (speed.0 * direction.0), transform.translation.z);
                    speed.0 += 0.06;
                }
        }
    }
}
