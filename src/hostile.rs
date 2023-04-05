use std::time::Duration;

use bevy::prelude::{self, *};
use bevy_rapier2d::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::{attribute::Health, collision, loot, player::Player};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnRate(Duration::from_secs(2)))
            .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_system(spawn)
            .add_system(despawn_hostiles)
            .add_system(update_spawn_timer)
            .add_system(move_to_player);
    }
}

#[derive(Default, Component)]
pub struct Hostile;
#[derive(Debug, Clone, Resource)]
pub struct SpawnTimer(pub Timer);
#[derive(Debug, Clone, Resource)]
pub struct SpawnRate(Duration);

fn update_spawn_timer(mut timer: ResMut<SpawnTimer>, rate: Res<SpawnRate>) {
    if !rate.is_changed() {
        return;
    }

    timer.0.set_duration(rate.0);
}

fn spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut rng: ResMut<GlobalRng>,
    player: Query<&Transform, With<Player>>,
) {
    let player = player.single();
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let pos = {
            let x = rng.f32_normalized();
            let y = rng.f32_normalized();
            let val = Vec3 { x, y, z: 0. }.normalize();
            player.translation + val * 1000.
        };

        let transform = Transform::from_translation(pos);
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

fn move_to_player(
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