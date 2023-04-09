use bevy::prelude::{self, *};
use bevy_egui::{egui, EguiContexts};
use bevy_turborand::GlobalRng;

use crate::{
    attribute::{Health, MaxHealth},
    buff,
    player::{self, Player},
    GameState,
};

pub struct Plugin;
impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(select_power.in_set(OnUpdate(GameState::LevelUp)))
            .add_system(health)
            .add_system(restart.in_set(OnUpdate(GameState::Dead)));
    }
}

fn restart(
    mut contexts: EguiContexts,
    player: Query<Entity, With<Player>>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let player = player.single();
    egui::Area::new("death").show(contexts.ctx_mut(), |ui| {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    ui.set_height(700.);
                    ui.set_width(1000.);

                    ui.scope(|ui| {
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                        ui.visuals_mut().override_text_color = Some(egui::Color32::RED);

                        ui.label("You died!");
                    });

                    if ui.button("Restart").clicked() {
                        commands.entity(player).despawn_recursive();
                        player::spawn(commands);
                        game_state.set(GameState::Game);
                    };
                });
            },
        );
    });
}

fn health(mut contexts: EguiContexts, player: Query<(&Health, &MaxHealth), With<Player>>) {
    let (health, max) = player.single();
    egui::Area::new("health").show(contexts.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.set_height(500.);
            ui.set_width(900.);

            let health = health.0 as f32 / max.0 as f32;
            println!("health: {}", health);
            ui.add(
                egui::ProgressBar::new(health)
                    .text("Health")
                    .fill(egui::Color32::RED),
            );
        });
    });
}

fn select_power(
    player: Query<Entity, With<Player>>,
    mut contexts: EguiContexts,
    mut writer: EventWriter<buff::Apply>,
    mut choices: ResMut<buff::Choices>,
    mut game_state: ResMut<NextState<GameState>>,
    mut rng: ResMut<GlobalRng>,
) {
    if choices.remaining == 0 {
        game_state.set(GameState::Game);
        return;
    }

    if choices.inner.is_empty() {
        choices.randomize(3, &mut rng);
    }

    let player = player.single();
    let mut remaining = choices.remaining;
    egui::Area::new("levelup").show(contexts.ctx_mut(), |ui| {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
            |ui| {
                ui.set_height(500.);
                ui.set_width(900.);

                ui.horizontal(|ui| {
                    for (positive, negative) in choices.inner.iter() {
                        let text = format!(
                            "Improve {} by +{:.2}% \n\n\n\n Decrease {} by {:.2}%",
                            positive.affect,
                            positive.value * 100.,
                            negative.affect,
                            negative.value * 100.
                        );

                        if ui
                            .add_sized((300., 150.), egui::Button::new(text).wrap(true))
                            .clicked()
                        {
                            remaining = remaining.saturating_sub(1);
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
                        }
                    }
                });
            },
        );
    });

    if choices.remaining != remaining {
        choices.randomize(3, &mut rng);
    }

    choices.remaining = remaining;
}
