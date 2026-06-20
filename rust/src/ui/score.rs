use bevy::prelude::*;
use godot::classes::{CharacterBody2D, Label};
use godot_bevy::prelude::{GodotAccess, GodotNodeHandle};

use crate::character::{CharacterDeathMessage, player::{EnemyKillCount, Player}};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (score_tracker, update_score_label));
}

fn score_tracker(
    mut death_message_reader: MessageReader<CharacterDeathMessage>,
    mut enemy_kill_count_query: Query<&mut EnemyKillCount>,
) {
    for message in death_message_reader.read() {
        let Ok(mut enemy_kill_count) = enemy_kill_count_query.get_mut(message.source) else {
            return;
        };
        enemy_kill_count.count += 1;
    }
}

// todo: this function shall be "level state" dependent
fn update_score_label(
    player_query: Query<(&EnemyKillCount, &GodotNodeHandle), (With<Player>, Changed<EnemyKillCount>)>,
    mut godot: GodotAccess,
) {
    for (enemy_kill_count, handle) in &player_query {
        // based on player, the relative path to the HUD can be determined
        let Some(player_body) = godot.try_get::<CharacterBody2D>(*handle) else {
            continue;
        };

        let Some(mut score_label) = player_body.try_get_node_as::<Label>("../HUD/ScoreLabel") else {
            warn!("Could not find Label at ../HUD/ScoreLabel");
            continue;
        };
        score_label.set_text(&format!("Score: {}", enemy_kill_count.count));

    }
}