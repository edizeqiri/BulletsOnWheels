use bevy::app::App;
use bevy::prelude::{
    in_state, Changed, Commands, Component, IntoScheduleConfigs, Justify, Name, OnEnter,
    Query, Text2d, TextLayout, Transform, Update, Vec3, With
};

use crate::character::player::Player;
use crate::character::Health;
use crate::gamestate::start::PLAYER_DEFAULTS;
use crate::gamestate::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::RUNNING), spawn_health_display_system)
        .add_systems(
            Update,
            update_health_display_system.run_if(in_state(GameState::RUNNING))
        );
}

#[derive(Component)]
struct HealthText;

fn spawn_health_display_system(mut commands: Commands) {
    commands.spawn((
        Name::new("Health Display"),
        Text2d::new(format!(
            "Player Health: {}",
            PLAYER_DEFAULTS.max_health // todo: sth not right here ;)
        )),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_translation(Vec3::new(-400.0, -250.0, 0.0)),
        HealthText
    ));
}

fn update_health_display_system(
    mut text_query: Query<&mut Text2d, With<HealthText>>,
    health_query: Query<&Health, (With<Player>, Changed<Health>)>
) {
    for health in &health_query {
        for mut text in &mut text_query {
            text.0 = format!("Player Health: {}", health.current);
        }
    }
}
