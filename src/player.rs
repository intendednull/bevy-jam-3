use bevy::prelude::{self, *};
use bevy_rapier2d::prelude::*;

use crate::{
    attribute::{self, Experience, MoveSpeed},
    collision, GameState,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(move_player.in_set(OnUpdate(GameState::Game)));
    }
}

#[derive(Default, Component)]
pub struct Player;

fn setup(mut commands: Commands) {
    // Player
    let mut player = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });

    player.insert((
        Player,
        Experience {
            current: 0,
            cap: 100,
        },
        Collider::cuboid(20., 20.),
        GravityScale(0.),
        CollisionGroups::new(
            collision::group::PLAYER,
            collision::group::HOSTILE
                | collision::group::HOSTILE_PROJECTILE
                | collision::group::LOOT,
        ),
    ));

    attribute::insert_common(&mut player);
}

/// Move player with WASD
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &MoveSpeed), With<Player>>,
) {
    let (mut transform, speed) = query.single_mut();

    transform.translation.x -= keyboard_input.pressed(KeyCode::A) as i32 as f32 * speed.0;
    transform.translation.x += keyboard_input.pressed(KeyCode::D) as i32 as f32 * speed.0;
    transform.translation.y += keyboard_input.pressed(KeyCode::W) as i32 as f32 * speed.0;
    transform.translation.y -= keyboard_input.pressed(KeyCode::S) as i32 as f32 * speed.0;
}
