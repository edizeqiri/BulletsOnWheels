use bevy::app::App;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiContexts, EguiPrimaryContextPass};
use bevy_inspector_egui::egui;
use rand::{Rng, RngCore};

use crate::character::Health;
use crate::character::enemy::{Enemy, EnemyDeathMessage, EnemyType, create_enemy_bundle};
use crate::character::player::{Player, PlayerDeathMessage};
use crate::gamestate::GameState;
use crate::projectile::Projectile;
use crate::weapon::Weapons;
use crate::world::LevelState;
use crate::world::map::Level;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(EguiPrimaryContextPass, dev_ui)
        .add_systems(OnEnter(GameState::RUNNING), spawn_health_display_system)
        .add_systems(
            Update,
            update_health_display_system.run_if(in_state(GameState::RUNNING))
        );
}

const HEALTH_FONT_SIZE: f32 = 33.;
const HEALTH_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const HEALTH_TEXT_PADDING: Val = Val::Px(5.0);

#[derive(Component)]
struct HealthText;

fn spawn_health_display_system(mut commands: Commands) {
    commands.spawn((
        Name::new("Health Display"),
        Text::new("Player Health: "),
        TextFont {
            font_size: HEALTH_FONT_SIZE,
            ..default()
        },
        TextColor(HEALTH_COLOR),
        HealthText, // marker to find this text again
        Node {
            position_type: PositionType::Absolute,
            top: HEALTH_TEXT_PADDING,
            left: HEALTH_TEXT_PADDING,
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font_size: HEALTH_FONT_SIZE,
                ..default()
            },
            TextColor(HEALTH_COLOR),
        )]
    ));
}

fn update_health_display_system(
    text: Single<Entity, (With<HealthText>, With<Text>)>,
    health_query: Query<&Health, (With<Player>, Changed<Health>)>,
    mut writer: TextUiWriter
) {
    for health in &health_query {
        *writer.text(*text, 1) = health.current.to_string();
    }
}

#[cfg(debug_assertions)]
fn dev_ui(
    mut command: Commands,
    mut context: EguiContexts,
    player_query: Single<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    bullets: Query<Entity, With<Projectile>>,
    current_state: Res<State<LevelState>>,
    levels: Query<Entity, With<Level>>
) {
    egui::SidePanel::right("Dev Panel").show(context.ctx_mut().unwrap(), |ui| {
        ui.heading("Dev Panel");

        if ui.button("Spawn Enemy").clicked() {
            let mut rng = rand::rng();
            command.spawn(create_enemy_bundle(
                Transform::from_xyz(
                    rng.random_range(-100.0..100.0),
                    rng.random_range(-100.0..100.0),
                    0.
                ),
                Weapons::default(),
                10,
                Name::new(format!("Enemy{}", rng.next_u32())),
                EnemyType::default()
            ));
        }

        if ui.button("Clear Enemies").clicked() {
            enemy_query.iter().chain(bullets.iter()).for_each(|enemy| {
                command.write_message(EnemyDeathMessage { entity: enemy });
            });
        };

        if ui.button("Reset Game").clicked() {
            for level in levels {
                command.entity(level).despawn();
            }
            command.entity(player_query.entity()).despawn();
            command.set_state(GameState::START);
            command.set_state(LevelState::NONE);
        }

        let current = current_state.get();
        let mut mut_level = current.clone();
        egui::ComboBox::from_label("Take your pick")
            .selected_text(format!("{mut_level:?}"))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut mut_level, LevelState::ZERO, "ZERO");
                ui.selectable_value(&mut mut_level, LevelState::ONE, "ONE")
            });
        if mut_level != *current {
            command.set_state(mut_level);
        }

    });


}
