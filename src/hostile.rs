use std::time::Duration;

use bevy::prelude::{self, *};
use bevy_rapier2d::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::{
    attribute::{self, AttackSpeedTimer, Damage, Health, MaxHealth},
    collision, loot,
    player::Player,
    ui::ORANGE,
    GameState,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnRate(Duration::from_millis(500)))
            .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .init_resource::<Score>()
            .add_system(despawn_all_hostiles.in_set(OnUpdate(GameState::Dead)))
            .add_systems(
                (
                    spawn,
                    despawn_hostiles,
                    update_spawn_timer,
                    move_to_player,
                    attack_player,
                )
                    .in_set(OnUpdate(GameState::Game)),
            );
    }
}

#[derive(Default, Component)]
pub struct Hostile;
#[derive(Debug, Clone, Resource)]
pub struct SpawnTimer(pub Timer);
#[derive(Debug, Clone, Resource)]
pub struct SpawnRate(pub Duration);
#[derive(Debug, Default, Clone, Resource)]
pub struct Score(pub u128);

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
        let mut commands = commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: ORANGE.into(),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform,
            ..default()
        });

        commands.insert((
            Hostile,
            GravityScale(0.0),
            Velocity::default(),
            RigidBody::Dynamic,
            Collider::cuboid(15., 15.),
            Friction::coefficient(0.),
            CollisionGroups::new(
                collision::group::HOSTILE,
                collision::group::PLAYER_PROJECTILE
                    | collision::group::HOSTILE
                    | collision::group::PLAYER,
            ),
            ActiveEvents::COLLISION_EVENTS,
        ));

        attribute::insert_common(&mut commands);
    }
}

fn attack_player(
    context: Res<RapierContext>,
    mut hostiles: Query<(Entity, &Damage, &mut AttackSpeedTimer), With<Hostile>>,
    mut player: Query<(Entity, &mut Health), With<Player>>,
) {
    let (player, mut health) = player.single_mut();
    for (hostile, damage, mut timer) in hostiles.iter_mut() {
        let has_contact = context
            .contact_pair(hostile, player)
            .map(|pair| pair.has_any_active_contacts())
            .unwrap_or(false);

        if !has_contact || !timer.0.finished() {
            continue;
        }

        health.0 = health.0.saturating_sub(damage.0);

        timer.0.reset();
    }
}

fn despawn_all_hostiles(mut commands: Commands, query: Query<Entity, With<Hostile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
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

pub struct EnemyDeathEvent(pub Vec3);

fn despawn_hostiles(
    query: Query<(Entity, &MaxHealth, &Transform), (With<Hostile>, Changed<MaxHealth>)>,
    mut commands: Commands,
    mut loot_writer: EventWriter<loot::Event>,
    mut score: ResMut<Score>,
    mut ev_enemy_death: EventWriter<EnemyDeathEvent>,
) {
    for (entity, health, transform) in query.iter() {
        if health.0 <= 0 {
            ev_enemy_death.send(EnemyDeathEvent(transform.translation));
            //ev_enemy_death.send(EnemyDeathEvent());
            commands.entity(entity).despawn();
            score.0 += 10;
            loot_writer.send(loot::Event(transform.translation));
        }
    }
}
