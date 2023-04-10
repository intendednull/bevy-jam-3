use bevy_kira_audio::{Audio, AudioControl, AudioChannel};
use bevy::prelude::{*, self};

use crate::audio::BgmChannel;

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_bgm.on_startup());
    }
}

fn start_bgm(asset_server: Res<AssetServer>, bgm_audio: Res<AudioChannel<BgmChannel>>) {
    let bgm_track = asset_server.load("music_fun_funky_whistle_groove_loop.wav");
    let bgm = bgm_audio
        .play(bgm_track).looped()
        .handle();
    bgm_audio.set_volume(0.01);
}