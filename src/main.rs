use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(move_camera)
        .run();
}

#[derive(Default, Component)]
struct Player;
#[derive(Default, Component)]
struct Hostile;
#[derive(Default, Component)]
struct Speed(f64);

fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Player
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        })
        .insert((Player, Speed(2.5)));

    // Hostile
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        })
        .insert(Hostile);
}

/// Move camera to player position
fn move_camera(
    mut transforms: Query<&mut Transform>,
    player: Query<Entity, With<Player>>,
    camera: Query<Entity, With<Camera>>,
) {
    let player_transform = *transforms.get(player.single()).unwrap();
    let mut camera_transform = transforms.get_mut(camera.single()).unwrap();

    camera_transform.translation = player_transform.translation;
}

/// Move player with WASD
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let (mut transform, speed) = query.single_mut();

    transform.translation.x -= keyboard_input.pressed(KeyCode::A) as i32 as f32 * speed.0 as f32;
    transform.translation.x += keyboard_input.pressed(KeyCode::D) as i32 as f32 * speed.0 as f32;
    transform.translation.y += keyboard_input.pressed(KeyCode::W) as i32 as f32 * speed.0 as f32;
    transform.translation.y -= keyboard_input.pressed(KeyCode::S) as i32 as f32 * speed.0 as f32;
}
