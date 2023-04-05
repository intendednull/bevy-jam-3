use bevy::prelude::{self, *};

use crate::player::Player;

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(follow_player);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Move camera to player position
fn follow_player(
    mut transforms: Query<&mut Transform>,
    player: Query<Entity, With<Player>>,
    camera: Query<Entity, With<Camera>>,
    time: Res<Time>,
) {
    let player_transform = *transforms.get(player.single()).unwrap();
    let mut camera_transform = transforms.get_mut(camera.single()).unwrap();

    camera_transform.translation = camera_transform
        .translation
        .lerp(player_transform.translation, time.delta_seconds() * 20.);
}
