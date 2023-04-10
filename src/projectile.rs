use bevy::{
    prelude::{self, *},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::{
    attribute::{AttackRange, AttackSpeedTimer, Damage, DupChance, MaxHealth},
    collision,
    hostile::Hostile,
    player::Player,
    ui::OFFWHITE,
    GameState,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProjectileEvent>().add_systems(
            (
                spawn_projectile,
                despawn_dead_projectiles,
                handle_collision,
                detect_collisions,
                update_projectile_speed,
            )
                .in_set(OnUpdate(GameState::Game)),
        );
    }
}

#[derive(Debug, Clone, Component)]
pub struct Projectile {
    last_hit: Option<Entity>,
}
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
    mut player: Query<(Entity, &Transform, &ProjectileSpeed, &mut AttackSpeedTimer), With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_shoot: EventWriter<ShootEvent>,
) {
    let (_player_entity, player_transform, projectile_speed, mut timer) = player.single_mut();
    // Determine direction of projectile base on arrow keys on keyboard
    let mut direction = Vec2::new(0.0, 0.0);
    direction.x -= keyboard_input.pressed(KeyCode::Left) as i32 as f32;
    direction.x += keyboard_input.pressed(KeyCode::Right) as i32 as f32;
    direction.y += keyboard_input.pressed(KeyCode::Up) as i32 as f32;
    direction.y -= keyboard_input.pressed(KeyCode::Down) as i32 as f32;
    direction *= projectile_speed.0;

    if direction == Vec2::ZERO || !timer.0.finished() {
        return;
    }

    spawn(
        &mut commands,
        &mut meshes,
        &mut materials,
        direction,
        player_transform,
        None,
    );

    timer.0.reset();
    ev_shoot.send(ShootEvent());
}

fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    direction: Vec2,
    begin: &Transform,
    last_hit: Option<Entity>,
) {
    let color: prelude::Color = OFFWHITE.into();
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            ..default()
        })
        .insert((
            RigidBody::Dynamic,
            Collider::ball(5.0),
            GravityScale(0.0),
            Velocity::linear(direction),
            TransformBundle::from(*begin),
            Projectile { last_hit },
            CollisionGroups::new(
                collision::group::PLAYER_PROJECTILE,
                collision::group::HOSTILE,
            ),
        ));
}

fn detect_collisions(
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
                        projectile_events.send(ProjectileEvent {
                            projectile: *a,
                            target: hostile_entity,
                        });
                    }
                } else if player_projectile.get(*b).is_ok() {
                    if let Ok(hostile_entity) = hostile.get_mut(*a) {
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

#[allow(clippy::too_many_arguments)]
fn handle_collision(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut projectiles: Query<(&Transform, &Projectile, &mut Velocity)>,
    mut events: EventReader<ProjectileEvent>,
    mut health: Query<&mut MaxHealth>,
    mut rng: ResMut<GlobalRng>,
    damage: Query<&Damage, With<Player>>,
    player: Query<(&DupChance, &ProjectileSpeed), With<Player>>,
) {
    let (chance, proj_speed) = player.single();
    let damage = damage.single().0;
    for event in events.iter() {
        let Ok((transform, Projectile { last_hit }, mut velocity)) = projectiles.get_mut(event.projectile) else {
            continue;
        };

        if rng.f32() < chance.0 {
            let direction = random_direction(&mut rng) * proj_speed.0;
            spawn(
                &mut commands,
                &mut meshes,
                &mut materials,
                direction,
                transform,
                Some(event.target),
            );
        }

        if Some(event.target) == *last_hit {
            velocity.linvel = random_direction(&mut rng) * proj_speed.0;
            continue;
        }

        if let Ok(mut health) = health.get_mut(event.target) {
            health.0 = health.0.saturating_sub(damage);
        }

        commands.entity(event.projectile).despawn();
    }
}

fn update_projectile_speed(
    mut projectiles: Query<&mut Velocity, (With<Projectile>, Changed<Velocity>)>,
    mut player: Query<&mut ProjectileSpeed, With<Player>>,
) {
    let player = player.single_mut();
    for mut velocity in projectiles.iter_mut() {
        velocity.linvel = velocity.linvel.normalize() * player.0;
    }
}

fn random_direction(rng: &mut GlobalRng) -> Vec2 {
    let x = rng.f32();
    let y = rng.f32();
    Vec2::new(x, y).normalize()
}
