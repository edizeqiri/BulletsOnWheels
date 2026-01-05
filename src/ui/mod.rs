use bevy::app::App;
use bevy::prelude::{
    Changed, Commands, Component, IntoScheduleConfigs, Justify, OnEnter, Query, Reflect, Text2d,
    TextLayout, Transform, Update, Vec3, With, in_state
};

use crate::character::Health;
use crate::character::player::Player;
use crate::gamestate::GameState;
use crate::gamestate::start::PLAYER_DEFAULTS;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::RUNNING), spawn_health_display_system)
        .add_systems(
            Update,
            update_health_display_system.run_if(in_state(GameState::RUNNING))
        );
}

#[derive(Component)]
struct HealthText {
    text2d: Text2d
}

fn spawn_health_display_system(mut commands: Commands) {
    commands.spawn((
        HealthText {
            text2d: Text2d::new(format!(
                "Player Health: {}",
                PLAYER_DEFAULTS.max_health // todo: sth not right here ;)
            ))
        },
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_translation(Vec3::new(-400.0, -250.0, 0.0))
    ));
}

fn update_health_display_system(
    mut query: Query<(&Health, &mut HealthText), (With<Player>, Changed<Health>)>
) {
    for (health, mut health_text) in &mut query {
        health_text.text2d.0 = format!("Player Health: {}", health.current);
    }
}
