use bevy::{
    prelude::{self, *},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;

use crate::{
    attribute::{AttackRange, AttackSpeedTimer, Damage, MaxHealth},
    collision,
    hostile::Hostile,
    player::Player,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProjectileEvent>()
            .add_system(spawn_projectile)
            .add_system(despawn_dead_projectiles)
            .add_system(apply_damage)
            .add_system(detect_collisions);
    }
}

#[derive(Debug, Clone, Component)]
pub struct Projectile;
#[derive(Debug, Clone, Component)]
pub struct Parent(pub Entity);
#[derive(Debug, Clone, Component)]
pub struct ProjectileSpeed(pub f32);
#[derive(Debug, Clone)]
pub struct ProjectileEvent {
    pub projectile: Entity,
    pub target: Entity,
}

pub struct ShootEvent();

fn spawn_projectile(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(Entity, &Transform, &ProjectileSpeed, &mut AttackSpeedTimer), With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_shoot: EventWriter<ShootEvent>,
) {
    let (player_entity, player_transform, projectile_speed, mut timer) = player.single_mut();
    // Determine direction of projectile base on arrow keys on keyboard
    let mut direction = Vec2::new(0.0, 0.0);
    direction.x -= keyboard_input.pressed(KeyCode::Left) as i32 as f32;
    direction.x += keyboard_input.pressed(KeyCode::Right) as i32 as f32;
    direction.y += keyboard_input.pressed(KeyCode::Up) as i32 as f32;
    direction.y -= keyboard_input.pressed(KeyCode::Down) as i32 as f32;
    direction *= projectile_speed.0;

    // Spawn projectile
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

        ev_shoot.send(ShootEvent());
        timer.0.reset();
}

fn detect_collisions(
    mut commands: Commands,
    mut hostile: Query<Entity, With<Hostile>>,
    player_projectile: Query<Entity, With<Projectile>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut projectile_events: EventWriter<ProjectileEvent>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                if player_projectile.get(*a).is_ok() {
                    if let Ok(hostile_entity) = hostile.get_mut(*b) {
                        commands.entity(*a).despawn();
                        projectile_events.send(ProjectileEvent {
                            projectile: *a,
                            target: hostile_entity,
                        });
                    }
                } else if player_projectile.get(*b).is_ok() {
                    if let Ok(hostile_entity) = hostile.get_mut(*a) {
                        commands.entity(*b).despawn();
                        projectile_events.send(ProjectileEvent {
                            projectile: *b,
                            target: hostile_entity,
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

fn despawn_dead_projectiles(
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
    player: Query<(&Transform, &AttackRange), With<Player>>,
    mut commands: Commands,
) {
    let (transform, range) = player.single();
    for (entity, projectile_transform) in projectiles.iter() {
        let distance = projectile_transform
            .translation
            .distance(transform.translation);
        if distance > range.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn apply_damage(
    damage: Query<&Damage, With<Player>>,
    mut events: EventReader<ProjectileEvent>,
    mut health: Query<&mut MaxHealth>,
) {
    let damage = damage.single().0;
    for event in events.iter() {
        if let Ok(mut health) = health.get_mut(event.target) {
            health.0 = health.0.saturating_sub(damage);
        }
    }
}
