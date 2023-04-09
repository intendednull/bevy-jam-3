mod attribute;
mod buff;
mod camera;
mod collision;
mod hostile;
mod loot;
mod player;
mod projectile;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;
use bevy_turborand::RngPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Game,
    LevelUp,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RngPlugin::default())
        .add_plugin(EguiPlugin)
        .add_state::<GameState>()
        .add_plugin(camera::Plugin)
        .add_plugin(loot::Plugin)
        .add_plugin(player::Plugin)
        .add_plugin(projectile::Plugin)
        .add_plugin(hostile::Plugin)
        .add_plugin(buff::Plugin)
        .add_plugin(attribute::Plugin)
        .add_plugin(ui::Plugin)
        .run();
}
