use bevy_kira_audio::{*, Audio, AudioControl, prelude::{AudioEmitter, SpacialAudio, *}};
use bevy::prelude::{*, self};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(play_sfx);
        app.add_audio_channel::<BgmChannel>();
        app.add_audio_channel::<ShootingChannel>();
        app.add_audio_channel::<EnemyChannel>();
        app.add_audio_channel::<UiChannel>();
    }
}


#[derive(Default, Component)]
pub struct AudioPlayer;

#[derive(Resource)]
pub struct BgmChannel;

#[derive(Resource)]
pub struct ShootingChannel;

#[derive(Resource)]
pub struct EnemyChannel;

#[derive(Resource)]
pub struct UiChannel;

fn play_sfx(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    shooting_audio: Res<AudioChannel<ShootingChannel>>,
    enemy_audio: Res<AudioChannel<EnemyChannel>>,
    ui_audio: Res<AudioChannel<UiChannel>>,
    mut commands: Commands,
    mut ev_levelup: EventReader<crate::attribute::LevelUpEvent>,
    mut ev_enemy_death: EventReader<crate::hostile::EnemyDeathEvent>,
    mut ev_shoot: EventReader<crate::projectile::ShootEvent>,
//  mut ev_ui_select: EventReader<crate::CRATENAME::UiClickEvent>,
) {
    for ev in ev_levelup.iter() {
        ui_audio.set_volume(0.03);
        
        let sfx = asset_server.load("ui_level_up.wav");
        ui_audio.play(sfx);
    }

    for ev in ev_enemy_death.iter() {
        enemy_audio.set_volume(0.045);

        let enemy_loc = ev.0;
        let sfx = asset_server.load("monster_take_damage_1.wav");
        //dbg!(enemyLoc);
        let death_sound = enemy_audio
        .play(sfx)
        .handle();
        commands
            .spawn(SpatialBundle  {
                transform: Transform::from_translation(enemy_loc),
                visibility: Visibility::Hidden,
                ..default()
            })
            .insert(AudioEmitter {
                instances: vec![death_sound],
            });
    }

    for ev in ev_shoot.iter() {
        shooting_audio.set_volume(0.005);

        let sfx = asset_server.load("projectile_01.wav");
        let death_sound = shooting_audio
        .play(sfx)
        .handle();
    }

    // Create event for UI "Select"
    /*for ev in ev_ui_select.iter() {
    let sfx = asset_server.load("ui_menu_click.wav");
        ui_audio.play(sfx);
        ui_audio.set_volume(0.03);
    }*/
}