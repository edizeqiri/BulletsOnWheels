use bevy::prelude::*;
use godot::classes::{
    CharacterBody2D, Label, class_macros::private::virtuals::VideoStreamPlayback::play,
};
use godot_bevy::prelude::{GodotAccess, GodotNodeHandle, SceneTreeRef};
use std::fs;

use crate::{
    character::{
        CharacterDeathMessage,
        player::{EnemyKillCount, Player},
    },
    gamestate::ExitGameMessage,
    world::level_manager::CurrentLevel,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(load_high_score())
        .add_systems(Update, (score_tracker, update_score_label))
        .add_systems(Update, save_high_score);
}

use std::path::Path;

const HIGH_SCORE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/highscore.txt");

#[derive(Resource, Default)]
struct HighScore {
    count: u32,
}

fn score_tracker(
    mut death_message_reader: MessageReader<CharacterDeathMessage>,
    mut enemy_kill_count_query: Query<&mut EnemyKillCount>,
    mut high_score: ResMut<HighScore>,
) {
    for message in death_message_reader.read() {
        let Ok(mut enemy_kill_count) = enemy_kill_count_query.get_mut(message.source) else {
            return;
        };
        enemy_kill_count.count += 1;
        high_score.count = high_score.count.max(enemy_kill_count.count);
    }
}

// todo: this function shall be "level state" de
fn update_score_label(
    player_query: Query<&EnemyKillCount, (With<Player>, Changed<EnemyKillCount>)>,
    current_level: Res<CurrentLevel>,
    mut scene_tree: SceneTreeRef,
    high_score: Res<HighScore>,
) {
    let level_id = current_level.level_id;

    let Ok(enemy_kill_count) = player_query.single() else {
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

    score_label.set_text(&format!(
        "Score: {}  High Score: {}",
        enemy_kill_count.count, high_score.count
    ));
}

fn save_high_score(
    exit_game_message: MessageReader<ExitGameMessage>,
    mut player_death_message: MessageReader<CharacterDeathMessage>,
    enemy_kill_count_query: Query<(Entity, &EnemyKillCount), With<Player>>,
    high_score: Res<HighScore>,
) {
    let Ok((player_entity, score)) = enemy_kill_count_query.single() else {
        return;
    };

    let player_died = player_death_message
        .read()
        .any(|death_message| death_message.target == player_entity);

    if exit_game_message.is_empty() && !player_died {
        warn!("player still alive");
        return;
    }

    let high_score_path = Path::new(HIGH_SCORE_PATH);
    let current_high_score = high_score.count;

    // the check is technically redundant, but avoids constant writing to file
    if score.count >= current_high_score {
        info!("New High Score!!!");
        if let Err(error) = fs::write(high_score_path, score.count.to_string()) {
            warn!(
                "Could not write high score: {} to file {:?}",
                error, high_score_path
            );
        }
    }
}

fn load_high_score() -> HighScore {
    let Ok(value) = fs::read_to_string(HIGH_SCORE_PATH) else {
        warn!("Could not read high score from file: {}", HIGH_SCORE_PATH);
        return HighScore { count: 0 };
    };

    let Ok(count) = value.trim().parse::<u32>() else {
        warn!(
            "Could not parse high score from file: {}. Value was: {:?}",
            HIGH_SCORE_PATH,
            value.trim()
        );
        return HighScore { count: 0 };
    };

    HighScore { count }
}

