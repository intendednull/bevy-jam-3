use std::time::Duration;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{self, *},
};
use bevy_turborand::GlobalRng;

use crate::{buff, projectile::ProjectileSpeed};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_timer_with_attack_speed)
            .add_system(level_up);
    }
}

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

fn level_up(
    mut query: Query<(&mut Experience, Entity), Changed<Experience>>,
    mut writer: EventWriter<buff::Apply>,
    mut rng: ResMut<GlobalRng>,
) {
    for (mut experience, entity) in query.iter_mut() {
        if experience.current >= experience.cap {
            experience.current -= experience.cap;
            experience.cap *= 2;

            writer.send_batch([
                buff::Apply {
                    target: entity,
                    diff: buff::Diff::random(&mut rng),
                },
                buff::Apply {
                    target: entity,
                    diff: buff::Diff::random(&mut rng).neg(),
                },
            ]);
        }
    }
}
