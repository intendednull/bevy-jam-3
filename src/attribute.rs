use std::time::Duration;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{self, *},
};
use bevy_rapier2d::prelude::RapierConfiguration;

use crate::{
    buff,
    hostile::{Score, SpawnRate},
    projectile::ProjectileSpeed,
    GameState,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems((update_timers, increase_difficulty).in_set(OnUpdate(GameState::Game)))
            .add_system(update_timer_with_attack_speed)
            .add_system(level_up)
            .add_system(freeze_all_movement);
    }
}

#[derive(Default, Component)]
pub struct DupChance(pub f32);
#[derive(Default, Component)]
pub struct MoveSpeed(pub f32);
#[derive(Default, Component)]
pub struct Damage(pub i32);
#[derive(Default, Component)]
pub struct MaxHealth(pub i32);
#[derive(Default, Component)]
pub struct Health(pub i32);
#[derive(Debug, Clone, Component)]
pub struct AttackSpeedTimer(pub Timer);
#[derive(Debug, Clone, Component)]
pub struct AttackSpeed(pub Duration);
#[derive(Debug, Clone, Component)]
pub struct AttackRange(pub f32);
#[derive(Default, Component)]
pub struct Experience {
    pub current: u32,
    pub cap: u32,
}

pub fn insert_common(commands: &mut EntityCommands) {
    commands.insert((
        Damage(10),
        AttackRange(800.),
        AttackSpeedTimer(Timer::from_seconds(0.5, TimerMode::Once)),
        AttackSpeed(Duration::from_millis(100)),
        ProjectileSpeed(500.),
        MoveSpeed(2.5),
        MaxHealth(100),
        Health(100),
        DupChance(0.1),
    ));
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

fn freeze_all_movement(mut config: ResMut<RapierConfiguration>, game_state: Res<State<GameState>>) {
    if !game_state.is_changed() {
        return;
    }

    if game_state.0 != GameState::Game {
        config.physics_pipeline_active = false;
    } else {
        config.physics_pipeline_active = true;
    }
}

fn update_timers(mut query: Query<&mut AttackSpeedTimer>, time: Res<Time>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn level_up(
    _spawn_rate: ResMut<SpawnRate>,
    mut query: Query<&mut Experience, Changed<Experience>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut choices: ResMut<buff::Choices>,
) {
    for mut experience in query.iter_mut() {
        if experience.current >= experience.cap {
            experience.current -= experience.cap;
            experience.cap += 100;

            choices.remaining += 1;
            game_state.set(GameState::LevelUp);
        }
    }
}

fn increase_difficulty(mut spawn_rate: ResMut<SpawnRate>, score: Res<Score>) {
    if !score.is_changed() {
        return;
    }

    let base_duration = 0.5; // Base duration in seconds
    let score_factor = 0.01; // Determines the rate at which the difficulty increases with score

    let new_duration = base_duration / (1.0 + score_factor * score.0 as f32);
    spawn_rate.0 = Duration::from_secs_f32(new_duration);
}
