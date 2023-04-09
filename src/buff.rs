use std::{ops::Sub, time::Duration};

use bevy::prelude::{self, *};
use bevy_turborand::prelude::*;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::{
    attribute::{AttackRange, AttackSpeed, Damage, DupChance, Health, MaxHealth, MoveSpeed},
    hostile,
    projectile::ProjectileSpeed,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Choices>()
            .add_event::<Apply>()
            .add_system(apply);
    }
}

#[derive(EnumIter, Clone, Copy, Debug, Display, PartialEq, Eq)]
pub enum Affect {
    Health,
    Damage,
    #[strum(serialize = "Movement Speed")]
    MoveSpeed,
    #[strum(serialize = "Attack Speed")]
    AttackSpeed,
    #[strum(serialize = "Attack Range")]
    AttackRange,
    #[strum(serialize = "Projectile Duplication Chance")]
    DupChance,
    #[strum(serialize = "Projectile Speed")]
    ProjectleSpeed,
}

#[derive(Resource, Default)]
pub struct Choices {
    pub inner: Vec<(Diff, Diff)>,
    pub remaining: u32,
}

impl Choices {
    pub fn random(count: u32, rng: &mut GlobalRng) -> Self {
        Self {
            inner: (0..count)
                .map(|_| {
                    let buff = Diff::random(rng, None);
                    let debuff = Diff::random(rng, Some(buff.affect)).neg();
                    (buff, debuff)
                })
                .collect(),
            remaining: 0,
        }
    }

    pub fn randomize(&mut self, count: u32, rng: &mut GlobalRng) {
        let remaining = self.remaining;
        *self = Self::random(count, rng);
        self.remaining = remaining;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Diff {
    pub affect: Affect,
    pub value: f32,
}

impl Diff {
    pub fn random(rng: &mut GlobalRng, skip: Option<Affect>) -> Self {
        let affect = rng
            .sample_iter(Affect::iter().filter(|a| Some(*a) != skip))
            .expect("Failed to sample affect");
        let values = [0.05, 0.1, 0.15, 0.2];
        let value = *rng.sample(&values).expect("Failed to sample value");
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
        &mut Health,
        &mut Damage,
        &mut MoveSpeed,
        &mut AttackSpeed,
        &mut AttackRange,
        &mut DupChance,
        &mut ProjectileSpeed,
    )>,
    _spawn_rate: ResMut<hostile::SpawnRate>,
) {
    for event in reader.iter() {
        if let Ok((
            mut max_health,
            mut health,
            mut damage,
            mut move_speed,
            mut attack_speed,
            mut attack_range,
            mut dup_chance,
            mut projectile_speed,
        )) = query.get_mut(event.target)
        {
            let percent = 1. + event.diff.value;
            match event.diff.affect {
                Affect::Health => {
                    max_health.0 = (((max_health.0 as f32) * percent) as i32).max(1);
                    health.0 = (((health.0 as f32) * percent) as i32).max(1);
                }
                Affect::Damage => damage.0 = (((damage.0 as f32) * percent) as i32).max(1),
                Affect::MoveSpeed => move_speed.0 *= percent,
                Affect::AttackSpeed => {
                    attack_speed.0 = Duration::from_secs_f32(
                        attack_speed.0.as_secs_f32() * (1. - percent.sub(1.)),
                    )
                }
                Affect::AttackRange => attack_range.0 = (attack_range.0 * percent).max(10.),
                Affect::DupChance => dup_chance.0 *= percent,
                Affect::ProjectleSpeed => projectile_speed.0 *= percent,
            }
        }
    }
}
