use std::time::Duration;

use bevy::prelude::{self, *};
use bevy_rapier2d::prelude::*;

use crate::{
    attribute::{AttackSpeed, AttackSpeedTimer, Damage, Experience, Range, Speed},
    collision,
    projectile::ProjectileSpeed,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(move_player);
    }
}

#[derive(Default, Component)]
pub struct Player;

fn setup(mut commands: Commands) {
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
            Damage(10),
            Range(800.),
        ));
}

/// Move player with WASD
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let (mut transform, speed) = query.single_mut();

    transform.translation.x -= keyboard_input.pressed(KeyCode::A) as i32 as f32 * speed.0;
    transform.translation.x += keyboard_input.pressed(KeyCode::D) as i32 as f32 * speed.0;
    transform.translation.y += keyboard_input.pressed(KeyCode::W) as i32 as f32 * speed.0;
    transform.translation.y -= keyboard_input.pressed(KeyCode::S) as i32 as f32 * speed.0;
}
