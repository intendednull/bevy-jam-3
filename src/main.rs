mod collision;
mod loot;

use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng, RngPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RngPlugin::default())
        .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_event::<ProjectileEvent>()
        .add_plugin(loot::Plugin)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(move_camera)
        .add_system(player_projectiles)
        .add_system(update_timer_with_attack_speed)
        .add_system(spawn_enemies)
        .add_system(move_hostiles_to_player)
        .add_system(detect_collisions)
        .add_system(handle_projectile_events)
        .add_system(despawn_hostiles)
        .run();
}

#[derive(Debug, Clone, Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Default, Component)]
struct Experience {
    pub current: u32,
    pub cap: u32,
}
#[derive(Default, Component)]
struct Player;
#[derive(Default, Component)]
struct Hostile;
#[derive(Default, Component)]
struct Speed(f32);
#[derive(Default, Component)]
struct Health(u32);
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
            Experience {
                current: 0,
                cap: 10,
            },
            Speed(2.5),
            Collider::cuboid(20., 20.),
            GravityScale(0.),
            AttackSpeedTimer(Timer::from_seconds(0.5, TimerMode::Once)),
            AttackSpeed(Duration::from_millis(100)),
            ProjectileSpeed(500.),
            CollisionGroups::new(
                collision::group::PLAYER,
                collision::group::HOSTILE
                    | collision::group::HOSTILE_PROJECTILE
                    | collision::group::LOOT,
            ),
        ));
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

fn update_timer_with_attack_speed(
    mut query: Query<(&mut AttackSpeedTimer, &AttackSpeed)>,
    time: Res<Time>,
) {
    for (mut timer, attack_speed) in query.iter_mut() {
        timer.0.set_duration(attack_speed.0);
        timer.0.tick(time.delta());
    }
}

fn player_projectiles(
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
                material: materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
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
                CollisionGroups::new(
                    collision::group::PLAYER_PROJECTILE,
                    collision::group::HOSTILE,
                ),
            ));

        timer.0.reset();
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut rng: ResMut<GlobalRng>,
    player: Query<&Transform, With<Player>>,
) {
    let player = player.single();
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        // TODO: Hostiles should only spawn off-screen
        let x = rng.f32_normalized() * 1000.;
        let y = rng.f32_normalized() * 1000.;

        let transform = Transform::from_translation(player.translation + Vec3 { x, y, z: 0. });
        // Hostile
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform,
                ..default()
            })
            .insert((
                Hostile,
                GravityScale(0.0),
                Velocity::default(),
                RigidBody::Dynamic,
                Collider::cuboid(20., 20.),
                Friction::coefficient(0.),
                CollisionGroups::new(
                    collision::group::HOSTILE,
                    collision::group::PLAYER_PROJECTILE
                        | collision::group::HOSTILE
                        | collision::group::PLAYER,
                ),
                ActiveEvents::COLLISION_EVENTS,
                Health(100),
            ));
    }
}

fn move_hostiles_to_player(
    mut query: Query<(&Transform, &mut Velocity), With<Hostile>>,
    player: Query<&Transform, With<Player>>,
) {
    let player_transform = player.single();

    for (transform, mut velocity) in query.iter_mut() {
        let direction = {
            let value = player_transform.translation - transform.translation;
            Vec2::new(value.x, value.y)
        };
        let direction = direction.normalize_or_zero();

        velocity.linvel = direction * 100.;
    }
}

struct ProjectileEvent {
    pub target: Entity,
    pub entity: Entity,
}

fn detect_collisions(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    hostile: Query<Entity, With<Hostile>>,
    player_projectile: Query<Entity, With<Projectile>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut projectile_event_writer: EventWriter<ProjectileEvent>,
) {
    let player = player.single();
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                if player == *a {
                    if hostile.contains(*b) {
                        // commands.entity(*b).despawn();
                    }
                } else if player == *b {
                    if hostile.contains(*a) {
                        // commands.entity(*a).despawn();
                    }
                } else if player_projectile.contains(*a) && hostile.contains(*b) {
                    projectile_event_writer.send(ProjectileEvent {
                        target: *b,
                        entity: *a,
                    });
                } else if player_projectile.contains(*b) && hostile.contains(*a) {
                    commands.entity(*b).despawn();
                    projectile_event_writer.send(ProjectileEvent {
                        target: *a,
                        entity: *b,
                    });
                }
            }
            _ => {}
        }
    }
}

fn handle_projectile_events(
    mut event_reader: EventReader<ProjectileEvent>,
    mut health: Query<&mut Health>,
    mut commands: Commands,
) {
    for event in event_reader.iter() {
        commands.entity(event.entity).despawn();
        if let Ok(mut health) = health.get_mut(event.target) {
            health.0 = health.0.saturating_sub(10);
        }
    }
}

fn despawn_hostiles(
    query: Query<(Entity, &Health, &Transform), (With<Hostile>, Changed<Health>)>,
    mut commands: Commands,
    mut loot_writer: EventWriter<loot::Event>,
) {
    for (entity, health, transform) in query.iter() {
        if health.0 == 0 {
            commands.entity(entity).despawn();
            loot_writer.send(loot::Event(transform.translation));
        }
    }
}
