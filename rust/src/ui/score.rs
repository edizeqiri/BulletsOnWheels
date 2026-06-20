use bevy::prelude::*;
use godot::classes::{CharacterBody2D, Label};
use godot_bevy::prelude::{GodotAccess, GodotNodeHandle, SceneTreeRef};

use crate::{character::{CharacterDeathMessage, player::{EnemyKillCount, Player}}, world::level_manager::CurrentLevel};

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

// todo: this function shall be "level state" de
fn update_score_label(
    player_query: Query<&EnemyKillCount, (With<Player>, Changed<EnemyKillCount>)>,
    current_level: Res<CurrentLevel>,
    mut scene_tree: SceneTreeRef,
) {
    let level_id = current_level.level_id;

    let Some(enemy_kill_count) = player_query.iter().next() else {
        warn!("no enemy kill count");
        return;
    };

    let score_label_path = format!("{}/HUD/ScoreLabel", level_id.root_node_path());

    let Some(root) = scene_tree.get().get_root() else {
        warn!("no root");
        return;
    };

    let Some(mut score_label) = root.try_get_node_as::<Label>(&score_label_path) else {
        warn!("Could not find Label at {}", score_label_path);
        return;
    };

    score_label.set_text(&format!("Score: {}", enemy_kill_count.count));
}