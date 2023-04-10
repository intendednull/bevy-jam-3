use bevy::{
    prelude::{self, *},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::{
    attribute::{Experience, MoveSpeed},
    collision,
    player::Player,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Event>()
            .add_system(drop_loot)
            .add_system(pickup_loot)
            .add_system(move_loot_to_player);
    }
}

pub struct Event(pub Vec3);

#[derive(Component)]
pub struct Loot;

fn pickup_loot(
    context: Res<RapierContext>,
    loot: Query<Entity, With<Loot>>,
    mut player: Query<(Entity, &mut Experience), With<Player>>,
    mut commands: Commands,
) {
    let (player, mut xp) = player.single_mut();
    for entity in loot.iter() {
        if let Some(contact_pair) = context.contact_pair(player, entity) {
            if contact_pair.has_any_active_contacts() {
                xp.current += 1;
                commands.entity(entity).despawn();
            }
        }
    }
}

fn move_loot_to_player(
    mut query: Query<(&mut Velocity, &Transform), With<Loot>>,
    player: Query<(&Transform, &MoveSpeed), With<Player>>,
    time: Res<Time>,
) {
    let (player, speed) = player.single();
    for (mut velocity, transform) in query.iter_mut() {
        let direction = {
            let val = (player.translation - transform.translation).normalize();
            Vec2::new(val.x, val.y)
        };
        velocity.linvel = velocity
            .linvel
            .lerp(direction * 5. * 200., time.delta_seconds() * 3.);
    }
}

fn drop_loot(
    mut events: EventReader<Event>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rng: ResMut<GlobalRng>,
) {
    for &Event(pos) in events.iter() {
        for _ in 0..rng.u32(10..30) {
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(5.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::YELLOW)),
                    ..Default::default()
                })
                .insert((
                    Loot,
                    RigidBody::Dynamic,
                    TransformBundle::from(Transform::from_translation(pos)),
                    Collider::cuboid(1., 1.),
                    ActiveEvents::COLLISION_EVENTS,
                    GravityScale(0.0),
                    Dominance::group(-1),
                    Velocity::linear(
                        Vec2::new(rng.i8(i8::MIN..i8::MAX) as _, rng.i8(i8::MIN..i8::MAX) as _)
                            .normalize()
                            * 1000.,
                    ),
                    CollisionGroups::new(collision::group::LOOT, collision::group::PLAYER),
                ));
        }
    }
}
