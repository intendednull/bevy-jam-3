use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(move_camera)
        .add_system(player_attack)
        .add_system(update_timer_with_attack_speed)
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
        .insert((
            Player,
            Speed(2.5),
            AttackSpeedTimer(Timer::from_seconds(0.5, TimerMode::Once)),
            AttackSpeed(Duration::from_millis(500)),
            ProjectileSpeed(500.),
        ));

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

#[derive(Debug, Clone, Component)]
pub struct AttackSpeedTimer(pub Timer);
#[derive(Debug, Clone, Component)]
pub struct AttackSpeed(Duration);
#[derive(Debug, Clone, Component)]
pub struct Projectile;
#[derive(Debug, Clone, Component)]
pub struct Parent(Entity);
#[derive(Debug, Clone, Component)]
pub struct ProjectileSpeed(f32);

fn update_timer_with_attack_speed(
    mut query: Query<(&mut AttackSpeedTimer, &AttackSpeed)>,
    time: Res<Time>,
) {
    for (mut timer, attack_speed) in query.iter_mut() {
        timer.0.set_duration(attack_speed.0);
        timer.0.tick(time.delta());
    }
}

fn player_attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut AttackSpeedTimer>,
    time: Res<Time>,
    player: Query<(Entity, &Transform, &ProjectileSpeed), With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (player_entity, player_transform, projectile_speed) = player.single();
    // Determine direction of projectile base on arrow keys on keyboard
    let mut direction = Vec2::new(0.0, 0.0);
    direction.x -= keyboard_input.pressed(KeyCode::Left) as i32 as f32;
    direction.x += keyboard_input.pressed(KeyCode::Right) as i32 as f32;
    direction.y += keyboard_input.pressed(KeyCode::Up) as i32 as f32;
    direction.y -= keyboard_input.pressed(KeyCode::Down) as i32 as f32;
    direction *= projectile_speed.0;

    // Spawn projectile
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());

        if direction == Vec2::ZERO || !timer.0.finished() {
            return;
        }

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            })
            .insert((
                RigidBody::Dynamic,
                Collider::ball(5.0),
                GravityScale(0.0),
                Velocity::linear(direction),
                TransformBundle::from(*player_transform),
                Parent(player_entity),
                Projectile,
            ));

        timer.0.reset();
    }
}
