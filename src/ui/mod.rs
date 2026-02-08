use bevy::app::App;
use bevy::prelude::*;

use crate::character::Health;
use crate::character::player::Player;
use crate::gamestate::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::RUNNING), spawn_health_display_system)
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
