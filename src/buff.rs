use std::{ops::Sub, time::Duration};

use bevy::prelude::{self, *};
use bevy_turborand::prelude::*;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::{
    attribute::{AttackRange, AttackSpeed, Damage, MaxHealth, MoveSpeed},
    hostile,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Choices>()
            .add_event::<Apply>()
            .add_system(apply);
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Display)]
pub enum Affect {
    Health,
    Damage,
    MoveSpeed,
    AttackSpeed,
    AttackRange,
    SpawnRate,
}

#[derive(Resource, Default)]
pub struct Choices(pub Vec<(Diff, Diff)>);

impl Choices {
    pub fn random(count: u32, rng: &mut GlobalRng) -> Self {
        Self(
            (0..count)
                .map(|_| (Diff::random(rng), Diff::random(rng).neg()))
                .collect(),
        )
    }

    pub fn randomize(&mut self, count: u32, rng: &mut GlobalRng) {
        *self = Self::random(count, rng);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Diff {
    pub affect: Affect,
    pub value: f32,
}

impl Diff {
    pub fn random(rng: &mut GlobalRng) -> Self {
        let affect = rng
            .sample_iter(Affect::iter())
            .expect("Failed to sample affect");
        let value = rng.f32();
        Self { affect, value }
    }

    pub fn neg(self) -> Self {
        Self {
            affect: self.affect,
            value: -self.value,
        }
    }
}

pub struct Apply {
    pub diff: Diff,
    pub target: Entity,
}

fn apply(
    mut reader: EventReader<Apply>,
    mut query: Query<(
        &mut MaxHealth,
        &mut Damage,
        &mut MoveSpeed,
        &mut AttackSpeed,
        &mut AttackRange,
    )>,
    mut spawn_rate: ResMut<hostile::SpawnRate>,
) {
    for event in reader.iter() {
        if let Ok((mut health, mut damage, mut move_speed, mut attack_speed, mut attack_range)) =
            query.get_mut(event.target)
        {
            let percent = 1. + event.diff.value;
            match event.diff.affect {
                Affect::Health => health.0 = (((health.0 as f32) * percent) as i32).max(1),
                Affect::Damage => damage.0 = (((damage.0 as f32) * percent) as i32).max(1),
                Affect::MoveSpeed => move_speed.0 *= percent,
                Affect::AttackSpeed => {
                    attack_speed.0 = Duration::from_secs_f32(
                        attack_speed.0.as_secs_f32() * (1. - percent.sub(1.)),
                    )
                }
                Affect::AttackRange => attack_range.0 = (attack_range.0 * percent).max(10.),
                Affect::SpawnRate => {
                    spawn_rate.0 =
                        Duration::from_secs_f32(spawn_rate.0.as_secs_f32() * (1. - percent.sub(1.)))
                }
            }
        }
    }
}