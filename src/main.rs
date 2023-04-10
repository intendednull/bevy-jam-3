mod attribute;
mod audio;
mod buff;
mod camera;
mod collision;
mod hostile;
mod loot;
mod music;
mod player;
mod projectile;
mod ui;

use attribute::LevelUpEvent;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::{*};
use bevy_rapier2d::prelude::*;
use bevy_turborand::RngPlugin;
use hostile::EnemyDeathEvent;
use projectile::ShootEvent;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Game,
    LevelUp,
    Dead,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RngPlugin::default())
        .add_plugin(EguiPlugin)
        .add_state::<GameState>()
        .add_plugin(AudioPlugin)
        .add_plugin(camera::Plugin)
        .add_plugin(loot::Plugin)
        .add_plugin(player::Plugin)
        .add_plugin(projectile::Plugin)
        .add_plugin(hostile::Plugin)
        .add_plugin(buff::Plugin)
        .add_plugin(attribute::Plugin)
        .add_plugin(ui::Plugin)
        .add_plugin(music::Plugin)
        .add_plugin(audio::Plugin)
        //.insert_resource(SpacialAudio { max_distance: 25. })
        .add_event::<LevelUpEvent>()
        .add_event::<ShootEvent>()
        .add_event::<EnemyDeathEvent>()
        .run();
}
