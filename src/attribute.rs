use std::time::Duration;

use bevy::prelude::{self, *};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_timer_with_attack_speed);
    }
}

#[derive(Default, Component)]
pub struct Speed(pub f32);
#[derive(Default, Component)]
pub struct Damage(pub u32);
#[derive(Default, Component)]
pub struct Health(pub u32);
#[derive(Debug, Clone, Component)]
pub struct AttackSpeedTimer(pub Timer);
#[derive(Debug, Clone, Component)]
pub struct AttackSpeed(pub Duration);
#[derive(Default, Component)]
pub struct Experience {
    pub current: u32,
    pub cap: u32,
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
