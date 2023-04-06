use bevy::prelude::{self, *};
use bevy_egui::{egui, EguiContexts};
use bevy_turborand::GlobalRng;

use crate::{buff, player::Player, GameState};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(select_power.in_set(OnUpdate(GameState::LevelUp)));
    }
}

fn select_power(
    player: Query<Entity, With<Player>>,
    mut contexts: EguiContexts,
    mut writer: EventWriter<buff::Apply>,
    choices: Res<buff::Choices>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let player = player.single();
    egui::Area::new("levelup").show(contexts.ctx_mut(), |ui| {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
            |ui| {
                ui.set_height(500.);
                ui.set_width(900.);

                ui.horizontal(|ui| {
                    for (positive, negative) in choices.0.iter() {
                        let text = format!(
                            "Improve {} by {:.2}% \n decrease {} by {:.2}%",
                            positive.affect,
                            positive.value * 100.,
                            negative.affect,
                            negative.value * 100.
                        );

                        if ui
                            .add_sized((300., 500.), egui::Button::new(text).wrap(true))
                            .clicked()
                        {
                            writer.send_batch([
                                buff::Apply {
                                    diff: *positive,
                                    target: player,
                                },
                                buff::Apply {
                                    diff: *negative,
                                    target: player,
                                },
                            ]);

                            game_state.set(GameState::Game);
                        }
                    }
                });
            },
        );
    });
}