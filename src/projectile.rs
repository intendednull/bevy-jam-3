use bevy::{
    prelude::{self, *},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;

use crate::{
    attribute::{AttackSpeedTimer, Damage, Health, Range},
    collision,
    hostile::Hostile,
    player::Player,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_projectile)
            .add_system(despawn_dead_projectiles)
            .add_system(detect_collisions);
    }
}

#[derive(Debug, Clone, Component)]
pub struct Projectile;
#[derive(Debug, Clone, Component)]
pub struct Parent(pub Entity);
#[derive(Debug, Clone, Component)]
pub struct ProjectileSpeed(pub f32);

fn spawn_projectile(
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

fn detect_collisions(
    mut commands: Commands,
    player: Query<&Damage, With<Player>>,
    mut hostile: Query<(Entity, &mut Health), With<Hostile>>,
    player_projectile: Query<Entity, With<Projectile>>,
    context: Res<RapierContext>,
) {
    let damage = player.single();
    for player_projectile in player_projectile.iter() {
        for (hostile, mut health) in hostile.iter_mut() {
            if let Some(contact_pair) = context.contact_pair(player_projectile, hostile) {
                if contact_pair.has_any_active_contacts() {
                    commands.entity(player_projectile).despawn();
                    health.0 = health.0.saturating_sub(damage.0);
                }
            }
        }
    }
}

fn despawn_dead_projectiles(
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
    player: Query<(&Transform, &Range), With<Player>>,
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
